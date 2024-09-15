use crate::geometry::Rect;
use std::fmt;

pub enum Color {
    Rgb(u8, u8, u8),
    Name(String),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Rgb(r, g, b) => write!(f, "#{r:02x}{g:02x}{b:02x}",),
            Color::Name(name) => write!(f, "{name}"),
        }
    }
}

#[derive(strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Format {
    Webp,
    Jpeg,
    Gif,
    Png,
    Avif,
    Heic,
}

pub enum Radius {
    Ellipsis(u32, u32),
    Circle(u32),
}

impl fmt::Display for Radius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Radius::Circle(radius) => write!(f, "{radius}"),
            Radius::Ellipsis(width, height) => write!(f, "{width}|{height}",),
        }
    }
}

#[derive(strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Filter {
    #[strum(serialize = "autojpg")]
    AutoJPG,
    BackgroundColor(Color),
    Blur {
        radius: u8,
        sigma: Option<u8>,
    },
    Brightness(i8),
    Contrast(i8),
    Convolution {
        matrix_items: Vec<i8>,
        number_of_columns: u8,
        should_normalize: bool,
    },
    Cover,
    Equalize,
    ExtractFocalPoints,
    Filling {
        color: Color,
        fill_transparent: bool,
    },
    Focal(Rect),
    Format(Format),
    Grayscale,
    MaxBytes(u32),
    NoUpscale,
    Noise(u8),
    Proportion(f32),
    Quality(u8),
    RedEye,
    Rgb {
        r_amount: i8,
        g_amount: i8,
        b_amount: i8,
    },
    Rotate(u16),
    RoundCorners {
        radius: Radius,
        color: Color,
        transparent: bool,
    },
    Saturation(i8),
    Sharpen {
        sharpen_amount: f32,
        sharpen_radius: f32,
        luminance_only: bool,
    },
    Stretch,
    StripEXIF,
    StripICC,
    Upscale,
    Watermark {
        image_url: String,
        x: i32,
        y: i32,
        alpha: u8,
        w_ratio: Option<u8>,
        h_ratio: Option<u8>,
    },
}

impl fmt::Display for Filter {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = match self {
            Filter::AutoJPG
            | Filter::Cover
            | Filter::Equalize
            | Filter::ExtractFocalPoints
            | Filter::Grayscale
            | Filter::NoUpscale
            | Filter::RedEye
            | Filter::Stretch
            | Filter::StripEXIF
            | Filter::StripICC
            | Filter::Upscale => vec![],
            Filter::BackgroundColor(color) => vec![color.to_string()],
            Filter::Brightness(brightness) => vec![brightness.to_string()],
            Filter::Contrast(contrast) => vec![contrast.to_string()],
            Filter::Focal(focal) => vec![focal.to_string()],
            Filter::Format(format) => vec![format.as_ref().to_string()],
            Filter::MaxBytes(n) => vec![n.to_string()],
            Filter::Noise(noise) => vec![noise.to_string()],
            Filter::Proportion(proportion) => vec![proportion.to_string()],
            Filter::Quality(quality) => vec![quality.to_string()],
            Filter::Rotate(rotate) => vec![rotate.to_string()],
            Filter::Saturation(saturation) => vec![saturation.to_string()],
            Filter::Blur { radius, sigma } => {
                let mut args = vec![radius.to_string()];
                if let Some(sigma) = sigma {
                    args.push(sigma.to_string());
                }
                args
            }
            Filter::Convolution {
                matrix_items,
                number_of_columns,
                should_normalize,
            } => vec![
                matrix_items
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(";"),
                number_of_columns.to_string(),
                should_normalize.to_string(),
            ],
            Filter::Filling {
                color,
                fill_transparent,
            } => {
                let mut args = vec![color.to_string()];
                if *fill_transparent {
                    args.push("1".to_string());
                }
                args
            }
            Filter::Rgb {
                r_amount,
                g_amount,
                b_amount,
            } => vec![
                r_amount.to_string(),
                g_amount.to_string(),
                b_amount.to_string(),
            ],
            Filter::RoundCorners {
                radius,
                color,
                transparent,
            } => {
                let mut args = vec![radius.to_string(), color.to_string()];
                if *transparent {
                    args.push("1".to_string());
                }
                args
            }
            Filter::Sharpen {
                sharpen_amount,
                sharpen_radius,
                luminance_only,
            } => vec![
                sharpen_amount.to_string(),
                sharpen_radius.to_string(),
                luminance_only.to_string(),
            ],
            Filter::Watermark {
                image_url,
                x,
                y,
                alpha,
                w_ratio,
                h_ratio,
            } => {
                let mut args = vec![
                    image_url.to_string(),
                    x.to_string(),
                    y.to_string(),
                    alpha.to_string(),
                ];
                if let Some(w_ratio) = w_ratio {
                    args.push(w_ratio.to_string());
                }
                if let Some(h_ratio) = h_ratio {
                    args.push(h_ratio.to_string());
                }
                args
            }
        };
        write!(f, "{}({})", self.as_ref(), args.join(","))
    }
}
