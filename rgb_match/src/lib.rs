pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        let a = self.a;
        
        match first {
            r => self.r = second,
            g => self.g = second,
            b => self.b = second,
            a => self.a = second,
            _ => panic!("Invalid color value: {}", first),
        }
        match second {
            r => self.r = first,
            g => self.g = first,
            b => self.b = first,
            a => self.a = first,
            _ => panic!("Invalid color value: {}", second),
        }
        self
    }
}