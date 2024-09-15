use filter::Filter;

use crate::{
    geometry::{Coords, Rect},
    server::Server,
};

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
pub struct Settings {
    #[builder(start_fn)]
    server: Server,
    response: Option<ResponseMode>,
    trim: Option<Trim>,
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
