use crate::settings::{Settings, SettingsBuilder};
use hmac::Hmac;
use sha1::Sha1;

pub type HmacSha1 = Hmac<Sha1>;

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

#[derive(Default, Clone)]
pub struct Server {
    pub origin: String,
    pub security: Security,
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

    pub fn url_builder(&self) -> SettingsBuilder {
        Settings::with_server(self.clone())
    }
}
