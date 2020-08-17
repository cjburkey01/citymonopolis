/// Represents some color that can be converted into other color types.
pub trait Color: From<RGBAColor> + Into<RGBAColor> {}

/// A color that has a red, green, blue, and alpha channel.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RGBAColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl RGBAColor {
    pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a: a as f32,
        }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::from_rgba(r, g, b, 1.0)
    }
}

impl Color for RGBAColor {}

impl Into<[f64; 4]> for RGBAColor {
    fn into(self) -> [f64; 4] {
        [self.r as f64, self.g as f64, self.b as f64, self.a as f64]
    }
}
impl Into<[f64; 3]> for RGBAColor {
    fn into(self) -> [f64; 3] {
        [self.r as f64, self.g as f64, self.b as f64]
    }
}
impl Into<[f32; 4]> for RGBAColor {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
impl Into<[f32; 3]> for RGBAColor {
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<[f64; 4]> for RGBAColor {
    fn from(array: [f64; 4]) -> Self {
        Self::from_rgba(array[0], array[1], array[2], array[3])
    }
}
impl From<[f64; 3]> for RGBAColor {
    fn from(array: [f64; 3]) -> Self {
        Self::from_rgb(array[0], array[1], array[2])
    }
}
impl From<[f32; 4]> for RGBAColor {
    fn from(array: [f32; 4]) -> Self {
        Self::from_rgba(
            array[0] as f64,
            array[1] as f64,
            array[2] as f64,
            array[3] as f64,
        )
    }
}
impl From<[f32; 3]> for RGBAColor {
    fn from(array: [f32; 3]) -> Self {
        Self::from_rgb(array[0] as f64, array[1] as f64, array[2] as f64)
    }
}
