use super::{Endpoint, EndpointBuilder};
use hmac::{digest::InvalidLength, Hmac, Mac};
use sha1::Sha1;

pub type HmacSha1 = Hmac<Sha1>;

#[derive(Default, Clone)]
pub enum Security {
    #[default]
    Unsafe,
    Hmac(HmacSha1),
}

impl TryFrom<String> for Security {
    type Error = InvalidLength;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let hmac = HmacSha1::new_from_slice(value.as_bytes())?;
        Ok(Security::Hmac(hmac))
    }
}

/// ```
/// use thumbor::Server;
///
/// let server = Server::new("http://localhost:8888", "my-security-key").unwrap();
/// ```
#[derive(Default, Clone)]
pub struct Server {
    pub origin: String,
    pub security: Security,
}

impl Server {
    pub fn new(origin: impl Into<String>, key: impl Into<String>) -> Result<Self, InvalidLength> {
        Ok(Server {
            origin: origin.into(),
            security: key.into().try_into()?,
        })
    }

    /// ```
    /// use thumbor::Server;
    ///
    /// // Don't use this in production !
    /// let server = Server::new_unsafe("http://localhost:8888");
    /// ```
    pub fn new_unsafe(origin: impl Into<String>) -> Self {
        Server {
            origin: origin.into(),
            security: Security::Unsafe,
        }
    }

    /// Create a new SettingsBuilder with the current Server.
    /// ```
    /// use thumbor::Server;
    ///
    /// let server = Server::new("http://localhost:8888", "my-security-key").unwrap();
    /// let builder = server.endpoint_builder();
    /// ```
    pub fn endpoint_builder(&self) -> EndpointBuilder {
        Endpoint::with_server(self.clone())
    }
}
