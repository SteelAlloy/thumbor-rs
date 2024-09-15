use crate::{
    geometry::{Coords, Rect},
    server::Server,
};
use filter::Filter;

mod builder;
pub mod filter;

#[derive(strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum HAlignment {
    Left,
    Center,
    Right,
}

#[derive(strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum VAlignment {
    Top,
    Middle,
    Bottom,
}

/// Removing surrounding space in images can be done using the trim option.
///
/// Unless specified trim assumes the top-left pixel color and no tolerance
/// (more on tolerance below).
///
/// Trim also supports color tolerance. The euclidean distance between the colors
/// of the reference pixel and the surrounding pixels is used. If the distance is
/// within the tolerance they’ll get trimmed. For a RGB image the tolerance would
/// be within the range 0-442.
#[derive(Default)]
pub enum Trim {
    #[default]
    TopLeft,
    BottomRight,
}

#[derive(Default)]
pub enum FitIn {
    #[default]
    Default,
    Adaptive,
    Full,
}

pub enum ResponseMode {
    Metadata,
    Debug,
}

#[derive(Default, bon::Builder)]
#[builder(start_fn = with_server)]
pub struct Endpoint {
    #[builder(start_fn)]
    server: Server,
    response: Option<ResponseMode>,
    trim: Option<Trim>,

    /// The manual crop is entirely optional. This is very useful for applications
    /// that provide custom real-time cropping capabilities to their users.
    ///
    /// The manual crop part of the url takes two points as arguments, separated by a colon.
    /// The first point is the left-top point of the cropping rectangle.
    /// The second point is the right-bottom point.
    ///
    /// This crop is performed before the rest of the operations, so it can be used as
    /// a prepare step before resizing and smart-cropping. It is very useful when you
    /// just need to get that celebrity face on a big picture full of people, as an example.
    #[builder(into)]
    crop: Option<Rect>,
    fit_in: Option<FitIn>,
    #[builder(into)]
    resize: Option<Coords>,
    h_align: Option<HAlignment>,
    v_align: Option<VAlignment>,
    #[builder(default, into)]
    filters: Vec<Filter>,
    #[builder(default)]
    smart: bool,
}
