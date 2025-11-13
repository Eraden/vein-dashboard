use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub vein_server_bind: String,
    pub vein_server_port: u32,
    #[serde(default)]
    pub refresh_interval_sec: Option<u64>,
}

impl Config {
    pub fn refresh_interval(&self) -> u64 {
        self.refresh_interval_sec.unwrap_or(5)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vein_server_bind: "0.0.0.0".into(),
            vein_server_port: 8080,
            refresh_interval_sec: Some(5),
        }
    }
}
