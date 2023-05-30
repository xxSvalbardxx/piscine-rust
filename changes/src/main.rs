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
