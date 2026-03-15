use std::fmt::Debug;

pub trait PeripheralCommand: Debug + Clone + Send + Sync + 'static {}
