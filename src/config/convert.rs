use std::collections::{HashMap, HashSet};

use tracing::error;

use super::description::ConditionDesc;
use crate::config::description::{ControllerDesc, SettingDesc};
use crate::controller::{Controller, Stage, StageSet};

impl ControllerDesc {
    pub fn initialize(self) -> Controller {
        let peripherals = self
            .peripherals
            .into_iter()
            .map(|(name, desc)| (name, desc.to_generic_real()))
            .collect::<HashMap<_, _>>();

        let mut stage_sets = Vec::new();

        for stage_set in self.stage_sets {
            let used_names = stage_set
                .stages
                .iter()
                .flat_map(|stage| stage.settings.keys().cloned())
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

                    let command_desc = stage.settings.remove(name);
                    let generic_command = command_desc.map(SettingDesc::into_any);

                    let command = peripheral.create_command(generic_command);
                    stage_commands.push(command);

                    default_commands.push(peripheral.create_command(None));
                }

                stages.push(Stage::new(
                    stage_commands,
                    Some(stage.condition.into_generic()),
                    stage.exit_condition.map(ConditionDesc::into_generic),
                    stage.name,
                ));
            }

            stage_sets.push(StageSet::new(
                stages,
                Stage::new(
                    default_commands,
                    None,
                    None,
                    format!("Default Stage for {}", stage_set.name.clone()),
                ),
                stage_set.name,
            ));
        }

        let joins = peripherals.into_values().map(|(_, join)| join).collect();

        Controller::new(stage_sets, joins)
    }
}
