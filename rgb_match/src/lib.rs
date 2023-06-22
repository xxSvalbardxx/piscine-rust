pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match first {
            self.r => self.r = second,
            self.g => self.g = second,
            self.b => self.b = second,
            self.a => self.a = second,
            _ => panic!("Invalid color value: {}", first),
        }
        match second {
            self.r => self.r = first,
            self.g => self.g = first,
            self.b => self.b = first,
            self.a => self.a = first,
            _ => panic!("Invalid color value: {}", second),
        }
        self
    }
}