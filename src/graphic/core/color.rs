use crate::maths::vector::Vect4f;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    pub fn black() -> Self {
        Self::new(0x00, 0x00, 0x00, 0xFF)
    }

    pub fn transparent() -> Self {
        Self::new(0x00, 0x00, 0x00, 0x00)
    }
}

impl Into<Vect4f> for Color {
    fn into(self) -> Vect4f {
        const CHANNEL_MAX: f32 = 0xFF as f32;
        Vect4f::new([
            self.red as f32 / CHANNEL_MAX,
            self.green as f32 / CHANNEL_MAX,
            self.blue as f32 / CHANNEL_MAX,
            self.alpha as f32 / CHANNEL_MAX
        ])
    }
}