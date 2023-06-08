#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    name: String,
    role: Role,
    age: u8,
}

impl Member {
    pub fn get_promotion() {}
    pub fn new(name: String, role: Role, age: u8) -> Boss {
        Member {
            name: name.to_string(),
            role: role,
            age: age,
        }
    }
}
