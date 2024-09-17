use std::fmt;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coords {
    x: i32,
    y: i32,
}

impl std::ops::Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Div<i32> for Coords {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Mul<i32> for Coords {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<f32> for Coords {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        #[allow(clippy::cast_precision_loss)]
        #[allow(clippy::cast_possible_truncation)]
        Self {
            x: (self.x as f32 * rhs) as i32,
            y: (self.y as f32 * rhs) as i32,
        }
    }
}

impl From<(i32, i32)> for Coords {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<[i32; 2]> for Coords {
    fn from([x, y]: [i32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<i32> for Coords {
    fn from(length: i32) -> Self {
        Self {
            x: length,
            y: length,
        }
    }
}

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

pub struct Rect {
    min: Coords,
    max: Coords,
}

impl Rect {
    pub fn new(a: Coords, b: Coords) -> Self {
        if a > b {
            Self { min: b, max: a }
        } else {
            Self { min: a, max: b }
        }
    }

    pub fn from_center(c: impl Into<Coords>, width: i32, height: i32) -> Self {
        let c = c.into();
        let rx = width / 2;
        let ry = height / 2;

        Self::from(((c.x - rx, c.y - ry), (c.x + rx, c.y + ry)))
    }

    #[must_use]
    pub fn center(&self) -> Coords {
        (self.min + self.max) / 2
    }

    pub fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> i32 {
        self.max.y - self.min.y
    }

    #[must_use]
    pub fn scale(mut self, factor: f32) -> Self {
        let center = self.center();

        self.min = center + (self.min - center) * factor;
        self.max = center + (self.max - center) * factor;

        self
    }
}

impl<T: Into<Coords>> From<(T, T)> for Rect {
    fn from((a, b): (T, T)) -> Self {
        Self::new(a.into(), b.into())
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.min, self.max)
    }
}
