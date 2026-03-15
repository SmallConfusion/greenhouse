use crate::{
    controller::{
        Controller, StageSet,
        stage::{self, Stage},
    },
    description::{ControllerDesc, PeripheralDesc, SettingDesc},
    peripheral::{
        command_preset::{CommandPreset, GenericPeripheral},
        peripheral_command::PeripheralCommand,
        running_peripheral::RunningPeripheral,
    },
    peripherals::{pin::Pin, vent::Vent},
};
use derive_more::{Constructor, From};
use std::{
    any::Any,
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};
use tokio::sync::watch::Sender;
use tracing::{error, trace, warn};

impl Controller {
    pub fn convert(desc: ControllerDesc) -> Self {
        let peripherals = desc
            .peripherals
            .into_iter()
            .map(|(name, desc)| (name, desc.to_generic_real()))
            .collect::<HashMap<_, _>>();

        let mut stage_sets = Vec::new();

        for stage_set in desc.stage_sets {
            let used_names = stage_set
                .stages
                .iter()
                .flat_map(|s| s.settings.keys().cloned())
                .collect::<HashSet<_>>();

            let mut stages = Vec::new();

            for mut stage in stage_set.stages {
                let stage_commands = used_names
                    .iter()
                    .filter_map(|name| {
                        let value = stage.settings.remove(name).map(SettingDesc::into_any);
                        let preset = peripherals.get(name).map(|(p, _)| p.create_command(value)); // TODO: This shit is smart and unreadable

                        if preset.is_none() {
                            error!("No peripheral with the name {name} is defined");
                        }

                        preset
                    })
                    .collect::<Vec<_>>();

                stages.push(Stage::new(stage_commands));
            }

            stage_sets.push(StageSet::new(stages));
        }

        Self::new(stage_sets)
    }
}
