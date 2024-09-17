use super::{Endpoint, Filters, Smart};
use crate::server::Security;
use base64ct::{Base64Url, Encoding};
use hmac::Mac;

impl Endpoint {
    fn build_path(&self, image_uri: impl ToString) -> String {
        let smart = self.smart.then_some(Smart);
        let filters = Filters::new(&self.filters);

        [
            self.response.as_ref().map(ToString::to_string),
            self.trim.as_ref().map(ToString::to_string),
            self.crop.as_ref().map(ToString::to_string),
            self.fit_in.as_ref().map(ToString::to_string),
            self.resize.as_ref().map(ToString::to_string),
            self.h_align.as_ref().map(ToString::to_string),
            self.v_align.as_ref().map(ToString::to_string),
            smart.as_ref().map(ToString::to_string),
            filters.as_ref().map(ToString::to_string),
            Some(image_uri.to_string()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join("/")
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
    pub fn to_path(&self, image_uri: impl ToString) -> String {
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
    pub fn to_url(&self, image_uri: impl ToString) -> String {
        format!("{}{}", self.server.origin, self.to_path(image_uri))
    }
}
