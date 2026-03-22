mod implementation;

pub use implementation::*;

pub trait Condition: std::fmt::Debug + Send {
    fn is_met(&self) -> bool;
}
