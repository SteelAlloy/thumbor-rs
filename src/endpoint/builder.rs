use super::Endpoint;
use crate::server::Security;
use base64ct::{Base64Url, Encoding};
use hmac::Mac;

impl Endpoint {
    fn build_path(&self, image_uri: &str) -> String {
        let mut path = vec![];

        if let Some(resp) = &self.response {
            path.push(resp.to_string());
        }

        if let Some(orientation) = &self.trim {
            path.push(orientation.to_string());
        }

        if let Some(crop) = &self.crop {
            path.push(crop.to_string());
        }

        if let Some(fit_in) = &self.fit_in {
            path.push(fit_in.to_string());
        }

        if let Some(resize) = &self.resize {
            path.push(resize.to_string());
        }

        if let Some(h_align) = &self.h_align {
            path.push(h_align.to_string());
        }

        if let Some(v_align) = &self.v_align {
            path.push(v_align.to_string());
        }

        if self.smart {
            path.push("smart".to_string());
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

    /// ```
    /// use thumbor::Server;
    ///
    /// let server = Server::new_unsafe("http://localhost:8888");
    /// let endpoint = server.endpoint_builder().build();
    /// let path = endpoint.to_path("path/to/my/image.jpg");
    ///
    /// assert_eq!(path, "/unsafe/path/to/my/image.jpg");
    /// ```
    pub fn to_path(&self, image_uri: &str) -> String {
        let path = self.build_path(image_uri);

        let security = match &self.server.security {
            Security::Unsafe => "unsafe".to_string(),
            Security::Hmac(hmac) => {
                let mut mac = hmac.clone();
                mac.update(path.as_bytes());

                let signature = mac.finalize().into_bytes();
                Base64Url::encode_string(&signature)
            }
        };

        format!("/{security}/{path}")
    }

    /// ```
    /// use thumbor::Server;
    ///
    /// let server = Server::new_unsafe("http://localhost:8888");
    /// let endpoint = server.endpoint_builder().build();
    /// let path = endpoint.to_url("path/to/my/image.jpg");
    ///
    /// assert_eq!(path, "http://localhost:8888/unsafe/path/to/my/image.jpg");
    /// ```
    pub fn to_url(&self, image_uri: &str) -> String {
        format!("{}{}", self.server.origin, self.to_path(image_uri))
    }
}
