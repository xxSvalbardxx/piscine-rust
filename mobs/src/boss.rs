
pub mod boss {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Boss {
        name: String,
        age: u8,
    }

    impl Boss {
        pub fn new(name: String, age: u8) -> Boss {
            Boss {
                name: name.to_string(),
                age: age,
            }
        }
    }
}
