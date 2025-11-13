use std::sync::Arc;

use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::{RwLock, broadcast};

use crate::{config::Config, logs::LogLine};

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServerEvent {
    Status(crate::game::event::Status),
    Logs(Vec<LogLine>),
    Players(Vec<crate::players::event::Player>),
}

impl From<crate::game::event::Status> for ServerEvent {
    fn from(value: crate::game::event::Status) -> Self {
        Self::Status(value)
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub config: Arc<RwLock<Config>>,
    pub client: reqwest::Client,
}

impl Client {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    pub async fn get<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        endpoint: &str,
    ) -> reqwest::Result<T> {
        let c = self.config.read().await;
        let res = self
            .client
            .get(format!(
                "{bind}:{port}{endpoint}",
                bind = c.vein_server_bind,
                port = c.vein_server_port
            ))
            .send()
            .await?
            .json()
            .await;

        if res.is_err() {
            let res: serde_json::Value = self
                .client
                .get(format!(
                    "{bind}:{port}{endpoint}",
                    bind = c.vein_server_bind,
                    port = c.vein_server_port
                ))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap_or_default();
            let txt = serde_json::to_string_pretty(&res).unwrap();
            let err = serde_json::from_str::<T>(&txt).err().unwrap();
            let line = err.line();

            let error_sector = txt
                .lines()
                .skip(line.saturating_sub(3))
                .take(6)
                .collect::<Vec<_>>()
                .join("\n");

            tracing::error!("{}", error_sector);
        }

        res
    }
}

pub struct ServerEventBus {
    _rx: broadcast::Receiver<ServerEvent>,
    tx: broadcast::Sender<ServerEvent>,
}

impl ServerEventBus {
    pub fn new() -> Self {
        let (tx, rx) = broadcast::channel(64);
        Self { tx, _rx: rx }
    }

    pub fn sender(&self) -> broadcast::Sender<ServerEvent> {
        self.tx.clone()
    }

    pub fn receiver(&self) -> broadcast::Receiver<ServerEvent> {
        self.tx.subscribe()
    }
}
