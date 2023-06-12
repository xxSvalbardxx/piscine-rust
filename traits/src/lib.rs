pub use std::fmt::Display;

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub strength: f64,
	pub score: i32,
	pub money: i32,
	pub weapons: Vec<String>,
}

pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player {
	pub fn eat<T>(&mut self, food: T) where T: Food {
		self.strength += food.gives();
	}

}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.0
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
    let pure_meat = self.weight_in_kg - (self.weight_in_kg * self.fat_content);
    ((self.weight_in_kg * self.fat_content )* 9.0) + (pure_meat * 4.0)
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "{}\nStrength: {}, Score: {}, Money: {}\nWeapons: {:?}",
        self.name, self.strength, self.score, self.money, self.weapons)
    }
}