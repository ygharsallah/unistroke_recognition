use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PointError {
    XOutOfRange { value: f32 },
    YOutOfRange { value: f32 },
    NotFinite,
}

impl Point {
    pub(crate) const fn from_coordinates(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn new(x: f32, y: f32) -> Result<Self, PointError> {
        match (x, y) {
            (x, y) if !x.is_finite() || !y.is_finite() => Err(PointError::NotFinite),
            (x, _) if !(-1.0..=1.0).contains(&x) => Err(PointError::XOutOfRange { value: x }),
            (_, y) if !(-1.0..=1.0).contains(&y) => Err(PointError::YOutOfRange { value: y }),
            _ => Ok(Self { x, y }),
        }
    }

    pub const fn x(&self) -> f32 {
        self.x
    }

    pub const fn y(&self) -> f32 {
        self.y
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
