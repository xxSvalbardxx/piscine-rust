use events::Event::*;
use chrono::Duration;

fn main() {
	let remainder = Remainder("Go to the doctor");
	println!("{}", remainder.notify());
	let registration = Registration(Duration::seconds(49094));
	println!("{}", registration.notify());
	let appointment = Appointment("Go to the doctor");
	println!("{}", appointment.notify());
	let holiday = Holiday;
	println!("{}", holiday.notify());
}
