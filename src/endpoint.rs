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
    /// The metadata endpoint has **ALL** the options that the image one has,
    /// but instead of actually performing the operations in the image, it just simulates the operations.
    ///
    /// Since it has the same options as the other endpoint, we won’t repeat all of them.
    /// To use the metadata endpoint, just add a /meta in the beginning of the url.
    ///
    /// Say we have the following crop URL:
    ///
    /// http://my-server.thumbor.org/unsafe/-300x-200/left/top/smart/path/to/my/nice/image.jpg
    ///
    /// If we want the metadata on what thumbor would do, just change the url to be
    ///
    /// http://my-server.thumbor.org/unsafe/meta/-300x-200/left/top/smart/path/to/my/nice/image.jpg
    ///
    /// After the processing is finished, thumbor will return a json object containing metadata
    /// on the image and the operations that would have been performed.
    ///
    /// The json looks like this:
    /// ```json
    /// {
    ///     thumbor: {
    ///         source: {
    ///             url: "path/to/my/nice/image.jpg",
    ///             width: 800,
    ///             height: 600
    ///         },
    ///         operations: [
    ///             {
    ///                 type: "crop",
    ///                 left: 10,
    ///                 top: 10,
    ///                 right: 300,
    ///                 bottom: 200
    ///             },
    ///             {
    ///                 type: "resize",
    ///                 width: 300,
    ///                 height: 200
    ///             },
    ///             { type: "flip_horizontally" },
    ///             { type: "flip_vertically" }
    ///         ]
    ///     }
    /// }
    /// ```
    Metadata,

    /// The debug endpoint helps debug focal points by drawing a rectangle around them.
    Debug,
}

/// An endpoint is a representation of a Thumbor image URL.
///
/// # Usage
///
/// [`EndpointBuilder`] is used to create an [`Endpoint`] instance.
///
/// You can then use the [`Endpoint::to_url`] method to generate the URL,
/// or the [`Endpoint::to_path`] method to get the path (without the server origin).
///
/// The format of the image URI depends heavily on the image loader you are using.
/// Thumbor comes pre-packaged with an HTTP loader and a Filesystem loader.
/// - If you use the HTTP loader, the URI corresponds to the image complete URI.
/// - If you use the Filesystem loader, the URI corresponds to the path of the image from the images root.
#[derive(Default, bon::Builder)]
#[builder(start_fn = with_server)]
pub struct Endpoint {
    #[builder(start_fn)]
    server: Server,
    response: Option<ResponseMode>,

    /// Removing surrounding space in images can be done using the trim option.
    ///
    /// Unless specified trim assumes the top-left pixel color and no tolerance
    /// (more on tolerance below).
    ///
    /// Trim also supports color tolerance. The euclidean distance between the colors
    /// of the reference pixel and the surrounding pixels is used. If the distance is
    /// within the tolerance they’ll get trimmed. For a RGB image the tolerance would
    /// be within the range 0-442.
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

    /// The fit-in argument specifies that the image should not be auto-cropped and
    /// auto-resized to be **EXACTLY** the specified size, and should be fit in an imaginary
    /// box of “E” width and “F” height, instead.
    ///
    /// Consider an image of *800px* x *600px*, and a fit of *300px* x *200px*.
    /// This is how thumbor would resize it:
    ///
    /// ![](https://thumbor.readthedocs.io/en/latest/_images/vertical-fit-in.png)
    ///
    /// Consider an image of *400px* x *600px*, and a fit of *300px* x *200px*.
    /// This is how thumbor would resize it:
    ///
    /// ![](https://thumbor.readthedocs.io/en/latest/_images/horizontal-fit-in.png)
    ///
    /// This is very useful when you need to fit an image somewhere, but you have no idea about the original image dimensions.
    ///
    /// If a full fit-in is used, instead of using the largest size for cropping it uses the smallest one, so in the above scenarios:
    ///
    /// For the image of *800px* x *600px*, with a full fit-in of *300px* x *200px*, we would get an image of *300px* x *225px*.
    ///
    /// For the image of *400px* x *600px*, with a full fit-in of *300px* x *200px*, we would get an image of *300px* x *450px*.
    fit_in: Option<FitIn>,
    #[builder(into)]
    resize: Option<Coords>,
    h_align: Option<HAlignment>,
    v_align: Option<VAlignment>,

    /// Thumbor allows for usage of a filter pipeline that will be applied sequentially to the image.
    #[builder(default, into)]
    filters: Vec<Filter>,

    /// Thumbor uses some very advanced techniques for obtaining important points of
    /// the image (referred to as Focal Points in the rest of this documentation).
    ///
    /// Even though Thumbor comes with facial recognition of Focal Points as well as
    /// feature recognition, you can easily implement your own detectors.
    ///
    /// If you use it in the url, smart cropping will be performed and will override both
    /// horizontal and vertical alignments if it finds any Focal Points.
    ///
    /// **The default value (in case it is omitted) for this option is not to use smart cropping.**
    #[builder(default)]
    smart: bool,
}
