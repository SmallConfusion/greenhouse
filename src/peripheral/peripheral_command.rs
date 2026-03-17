use std::fmt::{Debug, Display};

pub trait PeripheralCommand: Debug + Display + Clone + Send + Sync + 'static {}
