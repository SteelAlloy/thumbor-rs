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

/// How Filters Work
/// ----------------
///
/// Thumbor handles filters in a pipeline. This means that they
/// run sequentially in the order they are specified!
/// Given an original image with size $60x40$ and the
/// following transformations::
///
///    http://localhost:8888/fit-in/100x100/filters:watermark(..):blur(..):fill(red,1):upscale()/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
///
/// The resulting image will first check if it can fit into a $100x100$. Since it does,
/// the filter pipeline will kick in and:
///
/// * add the watermark in the image;
/// * blur the whole image (including the watermark);
/// * Fill the outer parts of the image with red (so it will fit in $100x100$);
/// * Then it will try to upscale. This will have no effect, since at this point the image is already $100x100$.
#[derive(strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Filter {
    /// AutoJPG
    /// =======
    ///
    /// <https://thumbor.readthedocs.io/en/latest/autojpg.html>
    ///
    /// Usage: `autojpg(enabled)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter overrides ``AUTO_PNG_TO_JPG`` config variable.
    ///
    /// Arguments
    /// ---------
    ///
    /// -  enabled - Passing ``True``, which is the default value, you will override the ``AUTO_PNG_TO_JPG`` config variable and False to keep the default behavior of thus config.
    ///
    /// Example
    /// -------
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/300x300/filters:autojpg()/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    #[strum(serialize = "autojpg")]
    AutoJPG,

    /// Background Color
    /// ================
    ///
    /// Usage: `background_color(color)`
    ///
    /// Description
    /// -----------
    ///
    /// The background_color filter sets the background layer to the specified color.
    /// This is specifically useful when converting transparent images (PNG) to JPEG
    ///
    /// Arguments
    /// ---------
    ///
    /// -  ``color`` - the color name (like in HTML) or hexadecimal rgb expression
    ///    without the "#" character (see
    ///    `<https://en.wikipedia.org/wiki/Web_colors>`_  for example). If color is
    ///    "auto", a color will be smartly chosen (based on the image pixels) to
    ///    be the filling color.
    ///
    /// Example
    /// -------
    ///
    /// The original image is:
    ///
    /// ![Original picture](https://thumbor.readthedocs.io/en/latest/_images/dice_transparent_background.png)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x300/filters:background_color(blue)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fdocs%2Fimages%2Fdice_transparent_background.png
    ///
    /// ![Picture after the background_color(blue) filter](https://thumbor.readthedocs.io/en/latest/_images/dice_blue_background.png)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x300/filters:background_color(f00)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fdocs%2Fimages%2Fdice_transparent_background.png
    ///
    /// ![Picture after the background_color(f00) filter](https://thumbor.readthedocs.io/en/latest/_images/dice_red_background.png)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x300/filters:background_color(add8e6)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fdocs%2Fimages%2Fdice_transparent_background.png
    ///
    /// ![Picture after the background_color(add8e6)](https://thumbor.readthedocs.io/en/latest/_images/dice_lightblue_background.png)
    BackgroundColor(Color),

    /// Blur
    /// ====
    ///
    /// Usage: `blur(radius [, sigma])`
    ///
    /// Description
    /// -----------
    ///
    /// This filter applies a gaussian blur to the image.
    ///
    /// Arguments
    /// ---------
    ///
    /// -  ``radius`` - Radius used in the gaussian function to generate a matrix,
    ///    maximum value is 150. The bigger the radius more blurred will be the
    ///    image.
    /// -  ``sigma`` - Optional. Defaults to the same value as the radius. Sigma
    ///    used in the gaussian function.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the blur filter](https://thumbor.readthedocs.io/en/latest/_images/blur_before.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:blur(7)/http%3A%2F%2Fupload.wikimedia.org%2Fwikipedia%2Fcommons%2Fthumb%2F8%2F8a%2F2006_Ojiya_balloon_festival_011.jpg%2F159px-2006_Ojiya_balloon_festival_011.jpg
    ///
    /// ![Picture after the blur filter](https://thumbor.readthedocs.io/en/latest/_images/blur_after.jpg)
    Blur { radius: u8, sigma: Option<u8> },

    /// Brightness
    /// ==========
    ///
    /// Usage: `brightness(amount)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter increases or decreases the image brightness.
    ///
    /// Arguments
    /// ---------
    ///
    /// - ``amount`` - ``-100 to 100`` - The amount (in %) to change the image brightness. Positive numbers make the image brighter and negative numbers make the image darker.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the brightness](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:brightness(40)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the brightness](https://thumbor.readthedocs.io/en/latest/_images/tom_after_brightness.jpg)
    Brightness(i8),

    /// Contrast
    /// ========
    ///
    /// Usage: `contrast(amount)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter increases or decreases the image contrast.
    ///
    /// Arguments
    /// ---------
    ///
    /// - ``amount`` - $-100$ to $100$ - The amount (in %) to change the image contrast. Positive numbers increase contrast and negative numbers decrease contrast.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the contrast filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:contrast(40)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after positive contrast](https://thumbor.readthedocs.io/en/latest/_images/tom_after_positive_contrast.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:contrast(-40)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after negative contrast](https://thumbor.readthedocs.io/en/latest/_images/tom_after_negative_contrast.jpg)
    Contrast(i8),

    /// Convolution
    /// ===========
    ///
    /// Usage: `convolution(matrix\_items, number\_of\_columns, should\_normalize)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter runs a convolution matrix (or kernel) on the image. See
    /// `Kernel (image
    /// processing) <http://en.wikipedia.org/wiki/Kernel_(image_processing)>`__
    /// for details on the process. Edge pixels are always extended outside the
    /// image area.
    ///
    /// Arguments
    /// ---------
    ///
    /// -  ``matrix_items`` - Semicolon separated matrix items.
    /// -  ``number_of_columns`` - Number of columns in the matrix.
    /// -  ``should_normalize`` - Whether or not we should divide each matrix item by the sum of all items.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the convolution filter](https://thumbor.readthedocs.io/en/latest/_images/before_convolution.png)
    ///
    /// Normalized Matrix:
    ///
    /// ::
    ///
    ///     1 2 1
    ///     2 4 2
    ///     2 1 2
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:convolution(1;2;1;2;4;2;1;2;1,3,true)/http://upload.wikimedia.org/wikipedia/commons/5/50/Vd-Orig.png
    ///
    /// ![Picture after the convolution filter](https://thumbor.readthedocs.io/en/latest/_images/after_convolution1.png)
    ///
    /// Matrix:
    ///
    /// ::
    ///
    ///     -1 -1 -1
    ///     -1  8 -1
    ///     -1 -1 -1
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:convolution(-1;-1;-1;-1;8;-1;-1;-1;-1,3,false)/http://upload.wikimedia.org/wikipedia/commons/5/50/Vd-Orig.png
    ///
    /// ![Picture after the convolution filter](https://thumbor.readthedocs.io/en/latest/_images/after_convolution2.png)
    Convolution {
        matrix_items: Vec<i8>,
        number_of_columns: u8,
        should_normalize: bool,
    },

    /// Cover
    /// =====
    ///
    /// Usage: `cover()`
    ///
    /// Description
    /// -----------
    ///
    /// This filter is used in GIFs to extract their first frame as the image to be used as cover.
    ///
    /// .. note:: This filter will only function when ``USE_GIFSICLE_ENGINE`` are set to ``True`` in ``thumbor.conf``:
    ///
    /// .. code:: python
    ///
    ///     USE_GIFSICLE_ENGINE = True
    ///
    /// Arguments
    /// ---------
    ///
    /// No arguments.
    ///
    /// Example
    /// -------
    ///
    /// ![Gif before cover filter](https://thumbor.readthedocs.io/en/latest/_images/animated.gif)
    ///
    /// `http://localhost:8888/unsafe/filters:cover()/http://server.my/animated_static.gif`
    ///
    /// ![Gif after cover filter](https://thumbor.readthedocs.io/en/latest/_images/animated_static.gif)
    Cover,

    /// Equalize
    /// ========
    ///
    /// Usage: `equalize()`
    ///
    /// Description
    /// -----------
    ///
    /// This filter equalizes the color distribution in the image.
    ///
    /// Arguments
    /// ---------
    ///
    /// No arguments.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the equalize filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:equalize()/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the equalize filter](https://thumbor.readthedocs.io/en/latest/_images/tom_after_equalize.jpg)
    Equalize,

    /// Extract focal points
    /// ====================
    ///
    /// Usage: `extract_focal()`
    ///
    /// Description
    /// -----------
    ///
    /// When cropping, thumbor uses focal points in the image to direct the area
    /// of the image that matters most. There are several ways of finding focal
    /// points. To learn more about focal points, visit the :doc:`detection_algorithms`.
    ///
    /// In order to use the ``extract_focal`` filter, the original image must be
    /// a thumbor URL that features manual cropping. To learn more about manual
    /// cropping, visit the :doc:`crop_and_resize_algorithms`.
    ///
    /// Using the original manual cropping points, this filter adds the cropped
    /// area (originally in the format `/LEFTxTOP:RIGHTxBOTTOM/`) as a focal point
    /// for the new image.
    ///
    /// For the new image, thumbor will use as the original the image URL that
    /// was the original for the segment with the manual cropping.
    ///
    /// This means that for an URL like:
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/300x100/filters:extract_focal()/localhost:8888/unsafe/100x150:300x200/https://upload.wikimedia.org/wikipedia/commons/thumb/2/22/Turkish_Van_Cat.jpg/546px-Turkish_Van_Cat.jpg
    ///
    /// Thumbor will use as original the following image URL:
    ///
    /// ::
    ///
    ///     https://upload.wikimedia.org/wikipedia/commons/thumb/2/22/Turkish_Van_Cat.jpg/546px-Turkish_Van_Cat.jpg
    ///
    /// Example
    /// -------
    ///
    /// Original Image:
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/2/22/Turkish_Van_Cat.jpg/546px-Turkish_Van_Cat.jpg)
    ///
    /// Cat's eye cropped:
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/100x150:300x200/https://upload.wikimedia.org/wikipedia/commons/thumb/2/22/Turkish_Van_Cat.jpg/546px-Turkish_Van_Cat.jpg
    ///
    /// ![](https://thumbor.readthedocs.io/en/latest/_images/extract1.jpg)
    ///
    /// A bigger image based on above's crop with the extract\_focal() filter:
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/300x100/filters:extract_focal()/localhost:8888/unsafe/100x150:300x200/https://upload.wikimedia.org/wikipedia/commons/thumb/2/22/Turkish_Van_Cat.jpg/546px-Turkish_Van_Cat.jpg
    ///
    /// ![](https://thumbor.readthedocs.io/en/latest/_images/extract2.jpg)
    ///
    /// Without the filter that would be the result:
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/300x100/localhost:8888/unsafe/100x150:300x200/https://upload.wikimedia.org/wikipedia/commons/thumb/2/22/Turkish_Van_Cat.jpg/546px-Turkish_Van_Cat.jpg
    ///
    /// ![](https://thumbor.readthedocs.io/en/latest/_images/extract3.jpg)
    ExtractFocalPoints,

    /// Filling
    /// =======
    ///
    /// Usage: `fill(color[,fill_transparent])`
    ///
    /// Description
    /// -----------
    ///
    /// This filter returns an image sized exactly as requested
    /// independently of its ratio. It will fill the missing area with the specified color.
    /// It is usually combined with the "fit-in" or "adaptive-fit-in" options.
    ///
    /// Arguments
    /// ---------
    ///
    /// -  ``color`` - the color name (like in HTML) or hexadecimal RGB expression
    ///    without the "#" character (see
    ///    `<https://en.wikipedia.org/wiki/Web_colors>`_ for example).
    ///
    ///    If color is "transparent" and the image format, supports transparency the
    ///    filling color is transparent.
    ///
    ///    <div class="warning">Some engines (like OpenCV engine) do not support transparency.</div>
    ///
    ///    If color is "auto", a color is smartly chosen (based on the image pixels)
    ///    as the filling color.
    ///
    ///    If color is "blur", the missing parts are filled with blurred original image.
    ///
    /// -  ``fill_transparent`` - a boolean to specify whether transparent areas of the
    ///    image should be filled or not. Accepted values are either `true`, `false`,
    ///    `1` or `0`. This argument is optional and the default value is `false`.
    ///
    /// Example #1
    /// ----------
    ///
    /// The original image is:
    ///
    /// ![Original picture](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x300/filters:fill(blue)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the fill(blue) filter](https://thumbor.readthedocs.io/en/latest/_images/fillblue.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x300/filters:fill(f00)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the fill(f00) filter](https://thumbor.readthedocs.io/en/latest/_images/fillred.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x300/filters:fill(add8e6)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the fill(add8e6)](https://thumbor.readthedocs.io/en/latest/_images/filllightblue.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x300/filters:fill(auto)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the fill(auto) filter (since 3.7.1)](https://thumbor.readthedocs.io/en/latest/_images/fillauto.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x300/filters:fill(blur)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the fill(blur) filter (since 6.7.1)](https://thumbor.readthedocs.io/en/latest/_images/fillblur.jpg)
    ///
    /// Example #2
    /// ----------
    ///
    /// The original image is:
    ///
    /// ![Original picture](https://thumbor.readthedocs.io/en/latest/_images/dice_transparent_background.png)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x225/filters:fill(blue,1)/https://github.com/thumbor/thumbor/wiki/dice_transparent_background.png
    ///
    /// ![Picture after the fill(blue) filter](https://thumbor.readthedocs.io/en/latest/_images/dice_blue_background.png)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x225/filters:fill(f00,true)/https://github.com/thumbor/thumbor/wiki/dice_transparent_background.png
    ///
    /// ![Picture after the fill(f00) filter](https://thumbor.readthedocs.io/en/latest/_images/dice_red_background.png)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x225/filters:fill(add8e6,1)/https://github.com/thumbor/thumbor/wiki/dice_transparent_background.png
    ///
    /// ![Picture after the fill(add8e6)](https://thumbor.readthedocs.io/en/latest/_images/dice_lightblue_background.png)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x225/filters:fill(auto,true)/https://github.com/thumbor/thumbor/wiki/dice_transparent_background.png
    ///
    /// ![Picture after the fill(auto) filter (since 3.7.1)](https://thumbor.readthedocs.io/en/latest/_images/dice_auto_background.png)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/300x225/filters:fill(blur,true)/https://github.com/thumbor/thumbor/wiki/dice_transparent_background.png
    ///
    /// ![Picture after the fill(blur) filter (since 6.7.1)](https://thumbor.readthedocs.io/en/latest/_images/dice_blur_background.png)
    Filling {
        color: Color,
        fill_transparent: bool,
    },

    /// Focal
    /// =====
    ///
    /// Usage: `focal(<left>x<top>:<right>x<bottom>)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter adds a focal point, which is used in later transforms.
    ///
    /// Arguments
    /// ---------
    ///
    /// -  ``left, top, right, bottom``: All mandatory arguments in the ``<left>x<top>:<right>x<bottom>`` format.
    ///
    /// Example
    /// -------
    ///
    ///
    /// Before cropping with specific focal point:
    ///
    /// ![Original picture](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/400x100/filters:focal(146x206:279x360)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// After specifying the focal point:
    ///
    /// ![Picture after the RGB filter](https://thumbor.readthedocs.io/en/latest/_images/after-focal.jpg)
    ///
    /// <div class="warning">When using this filter together with detectors, extract focal points filter or metadata parameter, unexpected behavior may occur.</div>
    Focal(Rect),

    ///Format
    ///======
    ///
    ///Usage: `format(image-format)`
    ///
    ///Description
    ///-----------
    ///
    ///This filter specifies the output format of the image. The output must be
    ///one of: "webp", "jpeg", "gif", "png", "avif" or "heic".
    ///
    ///Arguments
    ///---------
    ///
    ///- ``image-format`` - The output format of the resulting image.
    ///
    ///Example
    ///-------
    ///
    ///::
    ///
    ///    http://localhost:8888/unsafe/filters:format(webp)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    Format(Format),

    /// Grayscale
    /// =========
    ///
    /// Usage: `grayscale()`
    ///
    /// Description
    /// -----------
    ///
    /// This filter changes the image to grayscale.
    ///
    /// Arguments
    /// ---------
    ///
    /// No arguments.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the grayscale filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:grayscale()/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the grayscale filter](https://thumbor.readthedocs.io/en/latest/_images/tom_after_grayscale.jpg)
    Grayscale,

    /// Max bytes
    /// =========
    ///
    /// Usage: `max\_bytes(number-of-bytes)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter automatically degrades the quality of the image until the
    /// image is under the specified amount of bytes.
    ///
    /// Arguments
    /// ---------
    ///
    /// - ``number-of-bytes`` - The maximum number of bytes for the given image.
    ///
    /// Example
    /// -------
    ///
    /// Compressing the original image to less than 7.5k (ended up with ~7kb):
    ///
    /// ![Picture before the max_bytes filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:max_bytes(7500)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after 7500 max_bytes filter](https://thumbor.readthedocs.io/en/latest/_images/tom_after_max_bytes.jpg)
    MaxBytes(u32),

    /// No upscale
    /// ==========
    ///
    /// Usage: `no_upscale()`
    ///
    /// Description
    /// -----------
    ///
    /// This filter tells thumbor not to upscale your images.
    ///
    /// This means that if an original image is $300px$ width by $200px$ height and
    /// you ask for a $600x400$ image, thumbor will still return a $300x200$ image.
    ///
    /// Arguments
    /// ---------
    ///
    /// No arguments allowed.
    ///
    /// Example
    /// -------
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:no_upscale()/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    NoUpscale,

    /// Noise
    /// =====
    ///
    /// Usage: `noise(amount)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter adds noise to the image.
    ///
    /// Arguments
    /// ---------
    ///
    /// - ``amount`` - ``0% to 100%`` - The amount of noise to add to the image.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the noise filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:noise(40)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after noise of 40%](https://thumbor.readthedocs.io/en/latest/_images/tom_after_noise.jpg)
    Noise(u8),

    /// Proportion
    /// ==========
    ///
    /// Usage: `proportion(percentage)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter applies the specified proportion to the image's height and width when cropping.
    ///
    /// Arguments
    /// ---------
    ///
    /// -  ``percentage`` - The float percentage of the proportion (0.0 to 1.0).
    ///
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the percentage crop](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:proportion(0.5)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture with 50% crop](https://thumbor.readthedocs.io/en/latest/_images/proportion.jpg)
    Proportion(f32),

    /// Quality
    /// =======
    ///
    /// Usage: `quality(amount)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter changes the overall quality of the JPEG image (does nothing
    /// for PNGs or GIFs).
    ///
    /// Arguments
    /// ---------
    ///
    /// - ``amount`` - ``0 to 100`` - The quality level (in %) that the end image will
    /// feature.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the quality filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:quality(40)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after 10% quality](https://thumbor.readthedocs.io/en/latest/_images/tom_after_quality.jpg)
    Quality(u8),

    /// Red eye
    /// =======
    ///
    /// .. TODO: Document this filter
    ///
    /// Not documented yet
    RedEye,

    /// RGB
    /// ===
    ///
    /// Usage: `rgb(rAmount, gAmount, bAmount)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter changes the amount of color in each of the three channels.
    ///
    /// Arguments
    /// ---------
    ///
    /// -  ``rAmount`` - The amount of redness in the picture. Can range from -100
    ///    to 100 in percentage.
    /// -  ``gAmount`` - The amount of greenness in the picture. Can range from -100
    ///    to 100 in percentage.
    /// -  ``bAmount`` - The amount of blueness in the picture. Can range from -100
    ///    to 100 in percentage.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the RGB filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:rgb(20,-20,40)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the RGB filter](https://thumbor.readthedocs.io/en/latest/_images/tom_after_rgb.jpg)
    Rgb {
        r_amount: i8,
        g_amount: i8,
        b_amount: i8,
    },

    /// Rotate
    /// ======
    ///
    /// Usage: `rotate(angle)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter rotates the given image according to the angle value passed.
    ///
    /// .. note::
    ///     This filter rotates the image according to the engine.
    ///     For the PIL engine the rotation is done counter-clockwise.
    ///
    /// Arguments
    /// ---------
    ///
    /// - ``angle`` - ``0 to 359`` - The euler angle to rotate the image by. Numbers greater or equal than 360 will be transformed to a equivalent angle between 0 and 359.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the rotate filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:rotate(90)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the 90 degrees rotate](https://thumbor.readthedocs.io/en/latest/_images/tom_after_rotate.jpg)
    Rotate(u16),

    /// Round corners
    /// =============
    ///
    /// Usage: `round\_corner(a\|b,r,g,b,[transparent])`
    ///
    /// Description
    /// -----------
    ///
    /// This filter adds rounded corners to the image using the specified color
    /// as background.
    ///
    /// Arguments
    /// ---------
    ///
    /// - ``a|b`` - amount of pixels to use as radius. The argument ``b`` is not required, but it specifies the second value for the ellipsis used for the radius.
    /// - ``transparent`` - Optional. If set to true/1, the background will be transparent.
    ///
    /// Examples
    /// --------
    ///
    /// ![Picture before the round corners filter filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:round_corner(20,255,255,255)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after rounded corners](https://thumbor.readthedocs.io/en/latest/_images/rounded1.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:round_corner(20|40,0,0,0)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after rounded corners](https://thumbor.readthedocs.io/en/latest/_images/rounded2.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:round_corner(30,0,0,0,1)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after rounded corners (transparent)](https://thumbor.readthedocs.io/en/latest/_images/rounded3.png)
    RoundCorners {
        radius: Radius,
        color: Color,
        transparent: bool,
    },

    /// Saturation
    /// ========
    ///
    /// Usage: `saturation(amount)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter increases or decreases the image saturation.
    ///
    /// Arguments
    /// ---------
    ///
    /// - ``amount`` - $-100$ to $100$ - The amount (in %) to change the image saturation. Positive numbers increase saturation and negative numbers decrease saturation.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the saturation filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// `<http://localhost:8888/unsafe/filters:saturation(40)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg>`_
    ///
    /// ![Picture after positive saturation](https://thumbor.readthedocs.io/en/latest/_images/tom_after_positive_saturation.png)
    ///
    /// `<http://localhost:8888/unsafe/filters:saturation(-40)/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg>`_
    ///
    /// ![Picture after negative saturation](https://thumbor.readthedocs.io/en/latest/_images/tom_after_negative_saturation.png)
    Saturation(i8),

    /// Sharpen
    /// =======
    ///
    /// Usage: `sharpen(sharpen\_amount,sharpen\_radius,luminance\_only)`
    ///
    /// Description
    /// -----------
    ///
    /// This filter enhances apparent sharpness of the image. It's heavily based
    /// on Marco Rossini's excellent Wavelet sharpen GIMP plugin. Check
    /// `<http://registry.gimp.org/node/9836>`_ for details about how it work.
    ///
    /// Arguments
    /// ---------
    ///
    /// -  ``sharpen_amount`` - Sharpen amount. Typical values are between $0.0$ and
    ///    $10.0$.
    /// -  ``sharpen_radius`` - Sharpen radius. Typical values are between $0.0$ and
    ///    $2.0$.
    /// -  ``luminance_only`` - Sharpen only luminance channel. Values can be
    ///    ``true`` or ``false``.
    ///
    /// Example 1
    /// ---------
    ///
    /// ![Picture before the sharpen filter](https://thumbor.readthedocs.io/en/latest/_images/man_before_sharpen.png)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:sharpen(2,1.0,true)/http://videoprocessing.ucsd.edu/~stanleychan/research/pix/Blurred_foreman_0005.png
    ///
    /// ![Picture after the sharpen filter](https://thumbor.readthedocs.io/en/latest/_images/man_after_sharpen.png)
    ///
    /// Example 2
    /// ---------
    ///
    /// ![Picture before the sharpen filter](https://thumbor.readthedocs.io/en/latest/_images/eagle_before_sharpen.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:sharpen(1.5,0.5,true)/http://images.cambridgeincolour.com/tutorials/sharpening_eagle2-original.jpg
    ///
    /// ![Picture after the sharpen filter](https://thumbor.readthedocs.io/en/latest/_images/eagle_after_sharpen.jpg)
    ///
    Sharpen {
        sharpen_amount: f32,
        sharpen_radius: f32,
        luminance_only: bool,
    },

    /// Stretch
    /// =======
    ///
    /// Usage: `stretch()`
    ///
    /// Description
    /// -----------
    ///
    /// This filter stretches the image until it fits the required width and height, instead of cropping the image.
    ///
    /// Example
    /// -------
    ///
    /// ![Picture before the stretch filter](https://thumbor.readthedocs.io/en/latest/_images/tom_before_brightness.jpg)
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/200x100/filters:stretch()/https%3A%2F%2Fgithub.com%2Fthumbor%2Fthumbor%2Fraw%2Fmaster%2Fexample.jpg
    ///
    /// ![Picture after the stretch filter](https://thumbor.readthedocs.io/en/latest/_images/stretch_after.jpg)
    Stretch,

    /// Strip EXIF
    /// ==========
    ///
    /// Usage: `strip\_exif()`
    ///
    /// Description
    /// -----------
    ///
    /// This filter removes any Exif information in the resulting image. To keep the copyright information you have to set the configuration ``PRESERVE_EXIF_COPYRIGHT_INFO = True``.
    ///
    /// This is useful if you have set the configuration ``PRESERVE_EXIF_INFO = True`` but still wish to overwrite this behavior in some cases
    /// (e.g. for image icons)
    ///
    ///
    /// Arguments
    /// ---------
    ///
    /// No arguments
    ///
    /// Example
    /// -------
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:strip\_exif()/http://www.arte.tv/static-epgapi/057460-011-A.jpg
    StripEXIF,

    /// Strip ICC
    /// =========
    ///
    /// Usage: `strip\_icc()`
    ///
    /// Description
    /// -----------
    ///
    /// This filter removes any ICC information in the resulting image. Even
    /// though the image might be smaller, removing ICC information may result
    /// in loss of quality.
    ///
    /// Arguments
    /// ---------
    ///
    /// No arguments
    ///
    /// Example
    /// -------
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/filters:strip\_icc()/http://videoprocessing.ucsd.edu/~stanleychan/research/pix/Blurred_foreman_0005.png
    StripICC,

    /// Upscale
    /// =======
    ///
    /// Usage: `upscale()`
    ///
    /// Description
    /// -----------
    ///
    /// This filter tells thumbor to upscale your images. This only makes sense with
    /// "fit-in" or "adaptive-fit-in".
    ///
    /// This means that if an original image is $300px$ width by $200px$ height and you
    /// ask for a $600x500$ image, the filter will resize it to $600x400$.
    ///
    /// Arguments
    /// ---------
    ///
    /// No arguments allowed.
    ///
    /// Example
    /// -------
    ///
    /// ::
    ///
    ///     http://localhost:8888/unsafe/fit-in/600x500/filters:upscale()/https://raw.githubusercontent.com/thumbor/thumbor/e86324e49d7e53acc2a8057e43f3fdd2ca5cea75/docs/images/dice_transparent_background.png
    Upscale,

    /// Watermark
    /// =========
    ///
    /// Usage: `watermark(imageUrl, x, y, alpha [, w_ratio [, h_ratio]])`
    ///
    /// Description
    /// -----------
    ///
    /// This filter adds a watermark to the image. It can be positioned inside the image
    /// with the alpha channel specified and optionally resized based on the image size by
    /// specifying the ratio (see Resizing_).
    ///
    /// Arguments
    /// ---------
    ///
    /// -  ``imageUrl`` - Watermark image URL. It is very important to understand
    ///    that the same image loader that Thumbor uses will be used here. If
    ///    this URL contains parentheses they MUST be url encoded, since these
    ///    are the characters Thumbor uses as delimiters for filter parameters.
    /// -  ``x`` - Horizontal position that the watermark will be in. Positive
    ///    numbers indicate position from the left and negative numbers indicate
    ///    position from the right.
    ///    If the value is 'center' (without the single quotes), the watermark will be centered horizontally.
    ///    If the value is 'repeat' (without the single quotes), the watermark will be repeated horizontally.
    ///    If the value is a positive or negative number followed by a 'p' (ex. 20p) it will calculate the value
    ///    from the image width as percentage
    /// -  ``y`` - Vertical position that the watermark will be in. Positive numbers
    ///    indicate position from the top and negative numbers indicate position
    ///    from the bottom.
    ///    If the value is 'center' (without the single quotes), the watermark will be centered vertically.
    ///    If the value is 'repeat' (without the single quotes), the watermark will be repeated vertically
    ///    If the value is a positive or negative number followed by a 'p' (ex. 20p) it will calculate the value
    ///    from the image height as percentage
    /// -  ``alpha`` - Watermark image transparency. Should be a number between 0
    ///    (fully opaque) and 100 (fully transparent).
    /// -  ``w_ratio`` - percentage of the width of the image the watermark should fit-in, defaults to 'none'
    ///    (without the single quotes) which means it won't be limited in the width on resizing but also won't
    ///    be resized based on this value
    /// -  ``h_ratio`` - percentage of the height of the image the watermark should fit-in, defaults to 'none'
    ///    (without the single quotes) which means it won't be limited in the height on resizing but also won't
    ///    be resized based on this value
    ///
    /// Example
    /// -------
    ///
    /// ::
    ///
    ///     http://thumbor-server/filters:watermark(http://my.site.com/img.png,-10,-10,50)/some/image.jpg
    ///
    /// ![Picture after the watermark filter](https://thumbor.readthedocs.io/en/latest/_images/tom_after_watermark.jpg)
    ///
    /// ::
    ///
    ///     http://thumbor-server/filters:watermark(http://my.site.com/img.png,10p,-20p,50)/some/image.jpg
    ///
    /// ![Picture explaining watermark relative placement feature](https://thumbor.readthedocs.io/en/latest/_images/tom_watermark_relative.jpg)
    ///
    /// Resizing
    /// --------
    ///
    /// Resizing is being done by defining borders the watermark needs to fit in or being upscaled to.
    /// The ratio of the watermark will not be changed and will be expanded or shrinked to the size which
    /// fits best into the borders.
    ///
    /// Some examples are shown below with an original image having width=300 and height=200 and an imaginary
    /// watermark having width=30 and height=40. Borders are shown in red and the watermark drafted in green.
    ///
    /// Considering original image to be 300x200:
    ///
    /// - **watermark(imageUrl, 30, 10, 50, 20)**
    ///
    ///   20% of the *width*: 300px*0.2 = 60px so the original watermark *width* is 30px which means it
    ///   can be resized by 2.
    ///
    ///   Because the *height* isn't limited it can grow to 2x40px which is 80px.
    ///
    ///   ![Picture explaining watermark resizing feature](https://thumbor.readthedocs.io/en/latest/_images/tom_watermark_resized_width.jpg)
    ///
    /// - **watermark(imageUrl, 30, 10, 50, none, 15)**
    ///
    ///   15% of the *height*: 200px*0.15 = 30px so the original watermark *height* is 40px which means
    ///   it has to shrink by 25%.
    ///
    ///   Because the *width* isn't limited it can shrink to 0.75*30px which is 22.5px (rounded to 23px).
    ///
    ///   ![Picture explaining watermark resizing feature](https://thumbor.readthedocs.io/en/latest/_images/tom_watermark_resized_none_height.jpg)
    ///
    /// - **watermark(imageUrl, 30, 10, 50, 30, 30)**
    ///
    ///   30% of the *width*: 300px*0.3 = 90px
    ///
    ///   and
    ///
    ///   30% of the *height*: 200px*0.3 = 60px
    ///
    ///   so the original watermark *width* is 30px but cannot use 90px because then (to keep
    ///   the ratio) the *height* would need to become (40/30)*90px=120px but only 60px is allowed.
    ///
    ///   Therefor the *height* is limiting the resizing here and *height* would become 60px and *width*
    ///   would be (30/40)*60px=45px which fits into the 90px border.
    ///
    ///   ![Picture explaining watermark resizing feature](https://thumbor.readthedocs.io/en/latest/_images/tom_watermark_resized_width_height.jpg)
    Watermark {
        image_url: String,
        x: i32,
        y: i32,
        alpha: u8,
        w_ratio: Option<u8>,
        h_ratio: Option<u8>,
    },

    /// # Custom filter
    Custom { name: String, args: Vec<String> },
}

impl Filter {
    fn args(&self) -> Vec<String> {
        match self {
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
            Filter::Custom { args, .. } => args.clone(),
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
        }
    }
}

impl fmt::Display for Filter {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Filter::Custom { name, .. } => name,
            _ => self.as_ref(),
        };

        write!(f, "{name}({})", self.args().join(","))
    }
}
