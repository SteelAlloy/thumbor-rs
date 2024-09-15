use base64ct::{Base64Url, Encoding};
use hmac::Mac;
use http::Uri;

use crate::{
    error::Error,
    server::{HmacSha1, Security},
};

use super::{FitIn, ResponseMode, Settings, Trim};

impl Settings {
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
