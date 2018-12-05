#[derive(Copy, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Default for RGB {
    fn default() -> RGB {
        RGB {
            r: 0,
            g: 0,
            b: 0
        }
    }
}
