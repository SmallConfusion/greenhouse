use crate::{
    controller::{Controller, StageSet, stage::Stage},
    description::{ControllerDesc, PeripheralDesc, SettingDesc, StageSetDesc},
    peripheral::{
        self,
        command_preset::{CommandPreset, Peripheral},
        peripheral_command::PeripheralCommand,
        running_peripheral::RunningPeripheral,
    },
    peripherals::{
        pin::{Pin, PinState},
        vent::{Vent, VentState},
    },
};
use derive_more::{Constructor, From};
use std::{
    any::{self, Any, TypeId, type_name},
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    mem,
    rc::Rc,
};
use tokio::sync::watch::Sender;
use tracing::{error, trace, warn};

#[derive(Debug)]
struct PeripheralInfo<T: PeripheralCommand> {
    sender: Sender<T>,
    default: T,
}

#[derive(Debug, Constructor)]
struct PeripheralRequest<T: PeripheralCommand> {
    stage: Rc<RefCell<Stage>>,
    value: Option<T>,
}

#[derive(Debug, From)]
enum PeripheralRequestFor {
    Pin(Rc<RefCell<Stage>>, <Pin as Peripheral>::Command),
    Vent(Rc<RefCell<Stage>>, <Vent as Peripheral>::Command),
    Unknown(Rc<RefCell<Stage>>),
}

impl PeripheralRequestFor {
    pub fn give_peripheral<T: PeripheralCommand + 'static>(self, peripheral: PeripheralInfo<T>) {
        match self {
            PeripheralRequestFor::Pin(stage, value) => Self::give(&stage, value, peripheral),
            PeripheralRequestFor::Vent(stage, value) => Self::give(&stage, value, peripheral),

            PeripheralRequestFor::Unknown(stage) => Self::give_default(&stage, peripheral),
        }
    }

    fn give<T: PeripheralCommand + 'static, U: PeripheralCommand + 'static>(
        stage: &RefCell<Stage>,
        req_value: T,
        info: PeripheralInfo<U>,
    ) {
        let bv: Box<dyn Any> = Box::new(req_value);

        let final_value = match bv.downcast::<U>().map(|v| *v) {
            Ok(v) => v,
            Err(err) => {
                error!("Cannot give wrong type {err:?}");
                return;
            }
        };

        stage
            .borrow_mut()
            .add_command(CommandPreset::new(info.sender, final_value));
    }

    fn give_default<T: PeripheralCommand + 'static>(
        stage: &RefCell<Stage>,
        info: PeripheralInfo<T>,
    ) {
        stage
            .borrow_mut()
            .add_command(CommandPreset::new(info.sender, info.default));
    }
}

impl Controller {
    pub fn convert(desc: ControllerDesc) -> Self {
        let mut requests: HashMap<String, Vec<PeripheralRequestFor>> = HashMap::new();
        let mut sets: Vec<Vec<Rc<RefCell<Stage>>>> = Vec::new();

        for set in desc.stage_sets {
            let used_keys = set
                .stages
                .iter()
                .map(|s| s.settings.keys().cloned())
                .flatten()
                .collect::<Vec<String>>();

            let stages: Vec<Rc<RefCell<Stage>>> = Vec::new();
            sets.push(stages);
            let stages = sets.last_mut().unwrap();

            for mut stage in set.stages {
                let real_stage = Rc::new(RefCell::new(Stage::new()));
                stages.push(real_stage.clone());

                for key in used_keys.iter() {
                    let setting = stage.settings.remove(key);

                    match setting {
                        Some(SettingDesc::Pin(c)) => {
                            requests
                                .entry(key.clone())
                                .or_default()
                                .push((real_stage.clone(), c).into());
                        }

                        Some(SettingDesc::Vent(c)) => {
                            requests
                                .entry(key.clone())
                                .or_default()
                                .push((real_stage.clone(), c).into());
                        }

                        None => {
                            requests
                                .entry(key.clone())
                                .or_default()
                                .push(real_stage.clone().into());
                        }
                    }
                }
            }
        }

        for (name, desc) in desc.peripherals {
            trace!("Registering {name}");

            let Some(this_requests) = requests.remove(&name) else {
                warn!("{name} is defined but not used");
                continue;
            };

            let mut handles = Vec::new();

            match desc {
                PeripheralDesc::Pin(pin, pin_state) => {
                    let RunningPeripheral { sender, join } =
                        RunningPeripheral::create_from_peripheral(Pin::new(pin), pin_state);

                    for r in this_requests {
                        r.give_peripheral(PeripheralInfo {
                            sender: sender.clone(),
                            default: pin_state,
                        });
                    }

                    handles.push(join)
                }

                PeripheralDesc::Vent(vent_desc, default) => {
                    let RunningPeripheral { sender, join } =
                        RunningPeripheral::create_from_peripheral(
                            Vent::new(Pin::new(vent_desc.on), Pin::new(vent_desc.off)),
                            default.clone(),
                        );

                    for r in this_requests {
                        r.give_peripheral(PeripheralInfo {
                            sender: sender.clone(),
                            default: default.clone(),
                        });
                    }

                    handles.push(join)
                }
            }
        }

        let mut stage_sets = Vec::new();

        for set in sets {
            let mut new_set = Vec::new();

            for stage in set {
                match Rc::try_unwrap(stage) {
                    Ok(s) => new_set.push(s.into_inner()),
                    Err(e) => error!("Set has unresolved identifiers {e:?}"),
                }
            }

            stage_sets.push(StageSet::new(new_set));
        }

        Self::new(stage_sets)
    }
}
