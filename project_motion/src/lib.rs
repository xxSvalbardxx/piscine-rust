#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        let gravity = 9.8;
        let time_step = 1.0;

        self.time += time_step;
        let time_squared = self.time * self.time;

        self.actual_position.x =
            (self.init_position.x + self.init_velocity.x * self.time).round_to_precision(1);
        self.actual_position.y = (self.init_position.y + self.init_velocity.y * self.time
            - 0.5 * gravity * time_squared)
            .round_to_precision(1);

        self.actual_velocity.x = self.init_velocity.x;
        self.actual_velocity.y = (self.init_velocity.y - gravity * self.time).round_to_precision(1);

        if self.actual_position.y >= 0.0 {
            Some(self.clone())
        } else {
            None
        }
    }
}

pub trait RoundToPrecision {
    fn round_to_precision(self, precision: u32) -> f32;
}

impl RoundToPrecision for f32 {
    fn round_to_precision(self, precision: u32) -> f32 {
        let multiplier = 10.0_f32.powi(precision as i32);
        (self * multiplier).round() / multiplier
    }
}