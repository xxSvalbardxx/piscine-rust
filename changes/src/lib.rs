#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
        Self    {
        alias: alias.to_string(),
        brightness:0,
	    }
    }
}

pub fn change_brightness(lights: &mut Vec<Light>, alias: &str, value: u8) {
    for i in lights {
        if i.alias == alias{
            i.brightness = value
        }
    }   

}


#[cfg(test)]
mod tests {
    use changes::*;

fn main() {
	// bedroom
	let mut lights = vec![
		Light::new("living_room"),
		Light::new("bedroom"),
		Light::new("rest_room"),
	];
	println!("brightness = {}", lights[0].brightness);
	change_brightness(&mut lights, "living_room", 200);
	println!("new brightness = {}", lights[0].brightness);
}

}
