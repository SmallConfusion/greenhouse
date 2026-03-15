pub mod implementation;

pub trait Condition: std::fmt::Debug + Send {
    fn is_met(&self) -> bool;
}
