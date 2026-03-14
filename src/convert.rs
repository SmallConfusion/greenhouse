use std::collections::HashSet;

use crate::{
    controller::{Controller, StageSet},
    description::{ControllerDesc, StageSetDesc},
};

impl Controller {
    pub fn create_from_desc(mut desc: ControllerDesc) -> Self {
        todo!()
    }
}

impl StageSetDesc {
    pub fn used_keys(&self) -> HashSet<&String> {
        self.stages
            .iter()
            .map(|s| s.settings.keys())
            .flatten()
            .collect()
    }
}
