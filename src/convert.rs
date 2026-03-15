use crate::{
    controller::{Controller, stage::Stage, stage_set::StageSet},
    description::{ControllerDesc, SettingDesc},
};
use std::collections::{HashMap, HashSet};
use tracing::error;

pub fn convert_controller(desc: ControllerDesc) -> Controller {
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
        let mut default_commands = Vec::new();

        for mut stage in stage_set.stages {
            let mut stage_commands = Vec::new();

            for name in &used_names {
                let Some((peripheral, _handle)) = peripherals.get(name) else {
                    error!("Peripheral {name} is not defined");
                    continue;
                };

                let value = stage.settings.remove(name);
                let generic_value = value.map(SettingDesc::into_any);

                let command = peripheral.create_command(generic_value);
                stage_commands.push(command);

                default_commands.push(peripheral.create_command(None));
            }

            stages.push(Stage::new(
                stage_commands,
                Some(stage.condition.into_generic()),
                None,
            ));
        }

        stage_sets.push(StageSet::new(
            stages,
            Stage::new(default_commands, None, None),
        ));
    }

    let joins = peripherals.into_values().map(|(_, join)| join).collect();

    Controller::new(stage_sets, joins)
}
