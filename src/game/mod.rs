pub mod vein {
    use std::collections::HashMap;

    use crate::error::*;
    use crate::utils::Client;
    use serde::{Deserialize, Serialize};

    pub type VeinPlayerId = String;

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    pub struct VeinTime {
        #[serde(rename = "unixSeconds")]
        pub unix_seconds: u64,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinWeather {
        pub precipitation: f64,
        pub cloudiness: f64,
        pub temperature: f64,
        pub fog: f64,
        pub pressure: f64,
        pub relative_humidity: f64,
        pub wind_direction: f64,
        pub wind_force: f64,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinStatus {
        pub uptime: f64,
        pub online_players: HashMap<VeinPlayerId, VeinPlayerStatus>,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinPlayerStatus {
        pub character_id: String,
        pub time_connected: f64,
        pub name: String,
        pub status: String,
    }

    impl Client {
        pub async fn fetch_status(&self) -> Result<VeinStatus> {
            self.get("/status").await.map_err(Error::FetchStatus)
        }

        pub async fn fetch_weather(&self) -> Result<VeinWeather> {
            self.get("/weather").await.map_err(Error::FetchWeather)
        }

        pub async fn fetch_time(&self) -> Result<VeinTime> {
            self.get("/time").await.map_err(Error::FetchTime)
        }
    }
}

pub mod event {
    use std::collections::HashMap;

    use nutype::nutype;
    use serde::Serialize;

    use crate::players::event::*;

    #[nutype(derive(Serialize, Clone, PartialEq, Debug, Hash, Eq))]
    pub struct PlayerId(String);

    #[derive(Debug, Serialize, PartialEq, Clone)]
    pub struct PlayerStatus {
        pub character_id: CharacterId,
        pub time_connected: f64,
        pub name: String,
        pub status: String,
    }

    #[derive(Debug, Serialize, PartialEq, Clone)]
    pub struct Status {
        pub uptime: f64,
        pub human_uptime: String,
        pub online_players: HashMap<PlayerId, PlayerStatus>,
    }
}

pub mod agent {
    use std::collections::HashMap;

    use human_time::ToHumanTimeString;

    use crate::{
        config::Config,
        game::event::{PlayerId, PlayerStatus},
        players::event::CharacterId,
        utils::{Client, ServerEventBus},
    };

    pub fn start(client: Client, event_bus: &ServerEventBus, config: &Config) {
        let refresh_interval = config.refresh_interval();
        let tx = event_bus.sender();
        tokio::spawn(async move {
            let mut timeout =
                tokio::time::interval(tokio::time::Duration::from_secs(refresh_interval));
            loop {
                timeout.tick().await;
                let Ok(vein_status) = client
                    .fetch_status()
                    .await
                    .inspect_err(|e| tracing::warn!("{e}"))
                else {
                    continue;
                };

                let mut status = super::event::Status {
                    uptime: vein_status.uptime,
                    human_uptime: chrono::Duration::seconds(vein_status.uptime as i64)
                        .to_std()
                        .unwrap_or_default()
                        .to_human_time_string_with_format(
                            |n, unit| {
                                format!(
                                    "{n}{}",
                                    match unit {
                                        "d" => "days".to_owned(),
                                        "h" => "h".to_owned(),
                                        "m" => "m".to_owned(),
                                        "s" => "s".to_owned(),
                                        "ms" => "ms".to_owned(),
                                        other => other.to_string(),
                                    }
                                )
                            },
                            |acc, item| format!("{} {}", acc, item),
                        ),
                    online_players: HashMap::with_capacity(vein_status.online_players.capacity()),
                };
                for (player_id, player_status) in vein_status.online_players {
                    status.online_players.insert(
                        PlayerId::new(player_id.to_owned()),
                        PlayerStatus {
                            status: player_status.status,
                            time_connected: player_status.time_connected,
                            name: player_status.name,
                            character_id: CharacterId::new(player_status.character_id),
                        },
                    );
                }
                tx.send(status.into())
                    .inspect_err(|e| tracing::error!("Failed to send Event: {e}"))
                    .ok();
            }
        });
    }
}
