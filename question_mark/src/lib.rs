
pub struct One {
    // expected public fields
    pub first_layer: Option<Two>,
}
#[derive(Clone, Copy)]
pub struct Two {
    // expected public fields
    pub second_layer: Option<Three>,
}
#[derive(Clone, Copy)]
pub struct Three {
    // expected public fields
    pub third_layer: Option<Four>,
}
#[derive(Clone, Copy)]
pub struct Four {
    // expected public fields
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer?.second_layer?.third_layer?.fourth_layer
    }
}
