use std::fmt;

use figment::providers::{Format, Toml};
use figment::Figment;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Staging,
    Production,
}

impl Default for ENV {
    fn default() -> Self {
        Self::Development
    }
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "development"),
            ENV::Testing => write!(f, "testing"),
            ENV::Staging => write!(f, "staging"),
            ENV::Production => write!(f, "production"),
        }
    }
}

impl From<String> for ENV {
    fn from(env: String) -> Self {
        match env.as_str() {
            "development" => ENV::Development,
            "production" => ENV::Production,
            "testing" => ENV::Testing,
            _ => ENV::Development,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub db_url: String,
    pub mail_gun_sender_address: String,
    pub mail_gun_api_key: String,
    pub mail_gun_domain: String,
}

pub fn get_config() -> GlobalConfig {
    let env = ENV::from(std::env::var("ENV").unwrap_or("".to_string()));
    match env {
        ENV::Development => {
            let figment = Figment::new().merge(Toml::file("config/Dev.toml"));
            figment.extract::<GlobalConfig>().unwrap()
        }
        ENV::Testing => {
            let figment = Figment::new().merge(Toml::file("config/Test.toml"));
            figment.extract::<GlobalConfig>().unwrap()
        }
        ENV::Staging => GlobalConfig {
            db_url: "".to_string(),
            mail_gun_sender_address: "".to_string(),
            mail_gun_api_key: "".to_string(),
            mail_gun_domain: "".to_string(),
        },
        ENV::Production => GlobalConfig {
            db_url: "".to_string(),
            mail_gun_sender_address: "".to_string(),
            mail_gun_api_key: "".to_string(),
            mail_gun_domain: "".to_string(),
        },
    }
}
