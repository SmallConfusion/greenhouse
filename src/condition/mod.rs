pub trait Condition: std::fmt::Debug {
    fn is_met(&self) -> bool;
}

mod implementation {

    mod temperature {
        use crate::condition::Condition;

        #[derive(Debug)]
        pub struct TemperatureRange {
            min: f32,
            max: f32,
        }

        impl Condition for TemperatureRange {
            fn is_met(&self) -> bool {
                todo!()
            }
        }
    }
}
