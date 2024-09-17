use std::fmt;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct Point {
    #[serde(alias = "width")]
    x: i32,
    #[serde(alias = "height")]
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn flip_x(self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }

    pub fn flip_y(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Div<i32> for Point {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<f32> for Point {
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

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<[i32; 2]> for Point {
    fn from([x, y]: [i32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<i32> for Point {
    fn from(length: i32) -> Self {
        Self {
            x: length,
            y: length,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

#[derive(Deserialize, Debug)]
pub struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn from_center(c: impl Into<Point>, width: i32, height: i32) -> Self {
        let c = c.into();
        let rx = width / 2;
        let ry = height / 2;

        Self::from(((c.x - rx, c.y - ry), (c.x + rx, c.y + ry)))
    }

    pub fn left_top(&self) -> Point {
        Point::new(self.left, self.top)
    }

    pub fn right_bottom(&self) -> Point {
        Point::new(self.right, self.bottom)
    }

    #[must_use]
    pub fn center(&self) -> Point {
        (self.left_top() + self.right_bottom()) / 2
    }

    pub fn width(&self) -> i32 {
        self.bottom - self.top
    }

    pub fn height(&self) -> i32 {
        self.right - self.left
    }

    #[must_use]
    pub fn scale(mut self, factor: f32) -> Self {
        let center = self.center();

        self.left = center.x + ((self.left - center.x) as f32 * factor) as i32;
        self.right = center.x + ((self.right - center.x) as f32 * factor) as i32;
        self.top = center.y + ((self.top - center.y) as f32 * factor) as i32;
        self.bottom = center.y + ((self.bottom - center.y) as f32 * factor) as i32;

        self
    }
}

impl From<(i32, i32, i32, i32)> for Rect {
    fn from((left, top, right, bottom): (i32, i32, i32, i32)) -> Self {
        Self::new(left, top, right, bottom)
    }
}

impl From<[i32; 4]> for Rect {
    fn from([left, top, right, bottom]: [i32; 4]) -> Self {
        Self::new(left, top, right, bottom)
    }
}

impl<T: Into<Point>> From<(T, T)> for Rect {
    fn from((left_top, right_bottom): (T, T)) -> Self {
        let left_top = left_top.into();
        let right_bottom = right_bottom.into();
        Self::new(left_top.x, left_top.y, right_bottom.x, right_bottom.y)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.left_top(), self.right_bottom())
    }
}
