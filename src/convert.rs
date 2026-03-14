use std::collections::HashSet;

use crate::{
    controller::Controller,
    description::{ControllerDesc, StageSetDesc},
};

impl Controller {
    #[must_use] 
    pub fn create_from_desc(_desc: ControllerDesc) -> Self {
        todo!()
    }
}

impl StageSetDesc {
    #[must_use] 
    pub fn used_keys(&self) -> HashSet<&String> {
        self.stages
            .iter()
            .flat_map(|s| s.settings.keys())
            .collect()
    }
}
