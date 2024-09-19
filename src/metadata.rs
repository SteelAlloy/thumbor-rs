use crate::geometry::{Point, Rect};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub thumbor: Data,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub source: Source,
    pub operations: Vec<Operation>,
    pub target: Point,
    pub focal_points: Option<Vec<FocalPoint>>,
}

#[derive(Deserialize, Debug, Default)]
pub struct FocalPoint {
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub width: i32,
}

impl From<FocalPoint> for Rect {
    fn from(point: FocalPoint) -> Self {
        Rect::from_center((point.x, point.y), point.width, point.height)
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Operation {
    Resize(Point),
    Crop(Rect),
    FlipHorizontally,
    FlipVertically,
    AutoPngToJpgConversion,
}
