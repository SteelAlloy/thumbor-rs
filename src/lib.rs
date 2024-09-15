use base64ct::{Base64Url, Encoding};
use error::Error;
use geometry::{Coords, Rect};
use hmac::{Hmac, Mac};
use http::Uri;
use sha1::Sha1;

pub mod error;
pub mod filter;
pub mod geometry;

type HmacSha1 = Hmac<Sha1>;

#[derive(Default, Clone)]
pub enum Security {
    #[default]
    Unsafe,
    Hmac(String),
}

impl From<String> for Security {
    fn from(value: String) -> Self {
        Security::Hmac(value)
    }
}

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

#[derive(Default, Clone)]
pub struct Server {
    origin: String,
    security: Security,
}

impl Server {
    pub fn new_unsafe(origin: impl Into<String>) -> Self {
        Server {
            origin: origin.into(),
            security: Security::Unsafe,
        }
    }
    pub fn new_secured(origin: impl Into<String>, key: impl Into<String>) -> Self {
        Server {
            origin: origin.into(),
            security: Security::Hmac(key.into()),
        }
    }

    pub fn url_builder(&self) -> ThumborBuilder {
        Thumbor::with_server(self.clone())
    }
}

#[derive(Default, bon::Builder)]
#[builder(start_fn = with_server)]
pub struct Thumbor {
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
    filters: Vec<filter::Filter>,
    #[builder(default)]
    smart: bool,
}

impl Thumbor {
    fn build_path(&self, image_uri: &str) -> String {
        let mut path = vec![];

        if let Some(resp) = &self.response {
            path.push(
                match resp {
                    ResponseMode::Metadata => "meta",
                    ResponseMode::Debug => "debug",
                }
                .to_owned(),
            );
        }

        if let Some(orientation) = &self.trim {
            path.push(
                match orientation {
                    Trim::TopLeft => "trim:top-left",
                    Trim::BottomRight => "trim:bottom-right",
                }
                .to_owned(),
            );
        }

        if let Some(crop) = &self.crop {
            path.push(crop.to_string());
        }

        if let Some(fit_in) = &self.fit_in {
            path.push(
                match fit_in {
                    FitIn::Default => "fit-in",
                    FitIn::Adaptive => "adaptive-fit-in",
                    FitIn::Full => "full-fit-in",
                }
                .to_owned(),
            );
        }

        if let Some(resize) = &self.resize {
            path.push(resize.to_string());
        }

        if let Some(h_align) = &self.h_align {
            path.push(h_align.as_ref().to_owned());
        }

        if let Some(v_align) = &self.v_align {
            path.push(v_align.as_ref().to_owned());
        }

        if self.smart {
            path.push("smart".to_owned());
        }

        if !self.filters.is_empty() {
            let filters = self
                .filters
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(":");

            path.push(format!("filters:{filters}"));
        }

        path.push(image_uri.to_owned());

        path.join("/")
    }

    /// # Errors
    /// - `Error::UrlParseError`: URL parsing failed.
    /// - `Error::UrlCannotBeABase`: this URL is cannot-be-a-base.
    /// # Panics
    /// TODO: doc
    pub fn build(&self, image_uri: &str) -> Result<String, Error> {
        let path = self.build_path(image_uri);

        let security = match &self.server.security {
            Security::Unsafe => "unsafe".to_owned(),
            Security::Hmac(secret_key) => {
                let mut mac = HmacSha1::new_from_slice(secret_key.as_bytes()).unwrap();

                mac.update(path.as_bytes());

                let signature = mac.finalize().into_bytes();

                Base64Url::encode_string(&signature)
            }
        };

        Ok(format!("{}/{}/{}", self.server.origin, security, path))
    }

    /// # Errors
    /// - `Error::UrlParseError`: URL parsing failed.
    /// - `Error::UrlCannotBeABase`: this URL is cannot-be-a-base.
    /// # Panics
    /// TODO: doc
    pub fn build_uri(&self, image_uri: &str) -> Result<Uri, Error> {
        Ok(self.build(image_uri).unwrap().parse::<Uri>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use filter::Filter;
    use std::sync::LazyLock;

    const TEST_BASE: &str = "http://my.server.com";
    const SECURITY_KEY: &str = "my-security-key";
    const IMAGE_PATH: &str = "my.server.com/some/path/to/image.jpg";
    static SERVER: LazyLock<Server> =
        LazyLock::new(|| Server::new_secured(TEST_BASE, SECURITY_KEY));

    #[test]
    fn signing_of_a_known_url_results() {
        let width = 300;
        let height = 200;

        let builder = SERVER.url_builder().resize((width, height)).build();

        let uri = builder.build_uri(IMAGE_PATH).unwrap();

        assert_eq!(
            uri.path(),
            "/8ammJH8D-7tXy6kU3lTvoXlhu4o=/300x200/my.server.com/some/path/to/image.jpg"
        );
    }

    #[test]
    fn signature_with_meta() {
        let builder = SERVER
            .url_builder()
            .response(ResponseMode::Metadata)
            .build();

        let uri = builder.build_uri(IMAGE_PATH).unwrap();

        assert_eq!(
            uri.path(),
            "/Ps3ORJDqxlSQ8y00T29GdNAh2CY=/meta/my.server.com/some/path/to/image.jpg"
        );
    }

    #[test]
    fn signature_with_smart() {
        let builder = SERVER.url_builder().smart(true).build();

        let uri = builder.build_uri(IMAGE_PATH).unwrap();

        assert_eq!(
            uri.path(),
            "/-2NHpejRK2CyPAm61FigfQgJBxw=/smart/my.server.com/some/path/to/image.jpg"
        );
    }

    #[test]
    fn signature_with_fit_in() {
        let builder = SERVER.url_builder().fit_in(FitIn::Default).build();

        let uri = builder.build_uri(IMAGE_PATH).unwrap();

        assert_eq!(
            uri.path(),
            "/uvLnA6TJlF-Cc-L8z9pEtfasO3s=/fit-in/my.server.com/some/path/to/image.jpg"
        );
    }

    #[test]
    fn signature_with_filters() {
        let builder = SERVER
            .url_builder()
            .filters([Filter::Brightness(10), Filter::Contrast(20)])
            .build();

        let uri = builder.build_uri(IMAGE_PATH).unwrap();

        assert_eq!(uri.path(), "/ZZtPCw-BLYN1g42Kh8xTcRs0Qls=/filters:brightness(10):contrast(20)/my.server.com/some/path/to/image.jpg");
    }
}
