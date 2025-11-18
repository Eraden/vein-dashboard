use std::sync::Arc;

use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;

use crate::{config::Config, logs::LogLine};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Session {
    Disabled,
    Some(Uuid),
    None,
}

impl From<Option<Uuid>> for Session {
    fn from(value: Option<Uuid>) -> Self {
        match value {
            Some(uuid) => Self::Some(uuid),
            _ => Self::None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServerEvent {
    Status(crate::game::event::Status),
    Logs(Vec<LogLine>),
    Players(Vec<crate::players::event::Player>),
    Forbidden,
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
        let s = self
            .client
            .get(format!(
                "{bind}:{port}{endpoint}",
                bind = c.vein_server_bind,
                port = c.vein_server_port
            ))
            .send()
            .await?
            .text()
            .await?;

        let res = serde_json::from_str(&s);

        if let Err(ref e) = res {
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

            let to_skip = line.saturating_sub(5);
            let error_sector = txt
                .lines()
                .skip(to_skip)
                .take(10)
                .enumerate()
                .map(|(idx, s)| match idx {
                    _ if idx == line - to_skip - 1 => format!("{s} <<<<"),
                    _ => s.to_string(),
                })
                .collect::<Vec<_>>()
                .join("\n");

            tracing::error!("{e}\n{}", error_sector);
        }

        Ok(res.unwrap())
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
