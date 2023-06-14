#[allow(dead_code)]
pub struct Car<'a> {
	pub plate_nbr: &'a str,
	pub model: &'a str,
	pub horse_power: u32,
	pub year: u32,
}

#[allow(dead_code)]
pub struct Truck<'a> {
	pub plate_nbr: &'a str,
	pub model: &'a str,
	pub horse_power: u32,
	pub year: u32,
	pub load_tons: u32,
}

pub trait Vehicle {
	fn model(&self) -> &str;
	fn year(&self) -> u32;
}

impl Vehicle for Truck<'_> {
    // return the model of the truck
    fn model(&self) -> &str {
        self.model
    }
}

impl Vehicle for Car<'_> {
    // return the model of the car
    fn model(&self) -> &str {
        self.model
    }
}

fn all_models(list: Vec<&Vehicle>) -> Vec<&str> {
    let mut models = Vec::new();
    for vehicle in list {
        models.push(vehicle.model());
    }
    models
}