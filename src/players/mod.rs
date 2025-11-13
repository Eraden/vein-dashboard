pub mod vein {
    use crate::error::*;
    use crate::utils::Client;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinPlayers {
        pub players: Vec<String>,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinPlayerStats {
        pub start_time: String,
        pub stats_keys: Vec<String>,
        pub stats_values: Vec<f64>,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinPlayer {
        #[serde(rename = "iD")]
        pub id: String,
        pub character_ids: Vec<String>,
        pub first_join_time: String,
        pub last_join_time: String,
        pub last_name: String,
        pub b_server_muted: bool,
        pub stats: VeinPlayerStats,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinPlayerCharacterData {
        #[serde(rename = "iD")]
        pub id: String,
        // Character name
        pub name: String,
        pub character_gender: String,
        pub occupation: String,
        pub start_date: String,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinCharacterData {
        pub gender_identity: String,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinClothId {
        pub data: i64,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinCharacter {
        pub player_id: String,
        pub player_character_data: VeinPlayerCharacterData,
        pub character_data: VeinCharacterData,
        pub inventory: VeinCharacterInventory,
        pub conditions: Vec<VeinCharacterCondition>,
        pub clothes: Vec<VeinClothId>,
        pub addictions: Vec<serde_json::Value>,
        pub illnesses: Vec<serde_json::Value>,
        #[serde(rename = "hP")]
        pub hp: u16,
        pub consciousness: u16,
        #[serde(rename = "bUnconscious")]
        pub unconscious: bool,
        pub temperature: f64,
        pub stat_ids: Vec<String>,
        pub stat_values: Vec<serde_json::Value>,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinCharacterInventory {
        pub items: Vec<VeinCharacterInventoryItem>,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinCharacterInventoryItemData {
        pub data: Vec<String>,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinCharacterInventoryItemId {
        pub data: i64,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinCharacterInventoryItem {
        pub item: String,
        pub variant: i64,
        pub item_data: VeinCharacterInventoryItemData,
        pub acquisition_time: f64,
        pub slot: String,
        pub inventory: String,
        pub instance: String,
        pub damage_perc: f64,
        #[serde(rename = "iD")]
        pub id: VeinCharacterInventoryItemId,
        pub decay: i64,
        pub stack: i64,
        pub custom_label: String,
    }

    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct VeinCharacterCondition {
        pub condition_id: String,
        pub weight: f64,
    }

    impl Client {
        pub async fn fetch_character(&self, id: &str) -> Result<VeinCharacter> {
            self.get(&format!("/characters/{id}"))
                .await
                .map_err(Error::FetchCharacter)
        }
        pub async fn fetch_players(&self) -> Result<VeinPlayers> {
            self.get("/players").await.map_err(Error::FetchPlayers)
        }

        pub async fn fetch_player(&self, id: &str) -> Result<VeinPlayer> {
            self.get(&format!("/players/{id}"))
                .await
                .map_err(Error::FetchPlayer)
        }
    }
}

pub mod event {
    use crate::players::vein::{
        VeinCharacter, VeinCharacterInventoryItem, VeinCharacterInventoryItemData,
        VeinCharacterInventoryItemId, VeinPlayer, VeinPlayerCharacterData, VeinPlayerStats,
    };
    use inflector::Inflector;
    use nutype::nutype;
    use serde::Serialize;

    #[nutype(derive(Debug, Clone, Serialize, PartialEq, Eq, Hash))]
    pub struct PlayerId(String);

    #[nutype(derive(Debug, Clone, Serialize, PartialEq, Eq, Hash))]
    pub struct CharacterId(String);

    #[derive(Debug, PartialEq, Clone, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub struct CharacterInventoryItemId {
        pub data: i64,
    }

    impl From<VeinCharacterInventoryItemId> for CharacterInventoryItemId {
        fn from(value: VeinCharacterInventoryItemId) -> Self {
            let VeinCharacterInventoryItemId { data } = value;
            Self { data }
        }
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct PlayerCharacterData {
        pub id: String,
        // Character name
        pub name: String,
        pub character_gender: String,
        pub occupation: String,
        pub human_occupation: String,
        pub start_date: String,
    }

    impl From<VeinPlayerCharacterData> for PlayerCharacterData {
        fn from(value: VeinPlayerCharacterData) -> Self {
            let VeinPlayerCharacterData {
                id,
                name,
                character_gender,
                occupation,
                start_date,
            } = value;

            use inflector::Inflector;

            let human_occupation = occupation
                .strip_prefix("Occupation:O_")
                .map(String::from)
                .unwrap_or(occupation.clone())
                .to_title_case();

            Self {
                id,
                name,
                character_gender,
                occupation,
                human_occupation,
                start_date,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct Character {
        pub id: String,
        pub stats: Stats,
        pub inventory: Vec<CharacterInventoryItem>,
        pub player_character_data: PlayerCharacterData,
    }

    impl From<(String, VeinCharacter)> for Character {
        fn from((character_id, character): (String, VeinCharacter)) -> Self {
            Self {
                id: character_id,
                stats: (&character).into(),
                inventory: character
                    .inventory
                    .items
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                player_character_data: character.player_character_data.into(),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Default, Serialize)]
    pub struct CharacterInventoryItemData {
        pub data: Vec<String>,
    }

    impl From<VeinCharacterInventoryItemData> for CharacterInventoryItemData {
        fn from(value: VeinCharacterInventoryItemData) -> Self {
            let VeinCharacterInventoryItemData { data } = value;
            Self { data }
        }
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct CharacterInventoryItem {
        pub item: String,
        pub name: String,
        pub variant: i64,
        pub item_data: CharacterInventoryItemData,
        pub acquisition_time: f64,
        pub slot: String,
        pub inventory: String,
        pub instance: String,
        pub damage_perc: f64,
        pub id: CharacterInventoryItemId,
        pub decay: i64,
        pub stack: i64,
        pub custom_label: String,
    }

    impl From<VeinCharacterInventoryItem> for CharacterInventoryItem {
        fn from(value: VeinCharacterInventoryItem) -> Self {
            let VeinCharacterInventoryItem {
                item,
                variant,
                item_data,
                acquisition_time,
                slot,
                inventory,
                instance,
                damage_perc,
                id,
                decay,
                stack,
                custom_label,
            } = value;
            let name = item.split('/').last().unwrap_or_default();
            let name = name
                .split("_")
                .skip(1)
                .collect::<Vec<_>>()
                .join(" ")
                .split_once('.')
                .unwrap_or_default()
                .0
                .to_string();

            Self {
                name,
                item,
                variant,
                item_data: item_data.into(),
                acquisition_time,
                slot,
                inventory,
                instance,
                damage_perc,
                id: id.into(),
                decay,
                stack,
                custom_label,
            }
        }
    }

    #[derive(Debug, PartialEq, Default, Clone, Serialize)]
    #[serde(transparent)]
    pub struct Stats(Vec<Stat>);

    #[derive(Debug, PartialEq, Default, Clone, Serialize)]
    pub struct Stat {
        pub name: String,
        pub human_name: String,
        pub value: serde_json::Value,
    }

    impl<'vein> From<&'vein VeinCharacter> for Stats {
        fn from(character: &'vein VeinCharacter) -> Self {
            use inflector::Inflector;

            let stats = character
                .stat_ids
                .iter()
                .cloned()
                .zip(character.stat_values.iter().cloned())
                .map(|(name, value)| Stat {
                    human_name: name
                        .strip_prefix("ST_")
                        .map(String::from)
                        .unwrap_or_else(|| name.clone())
                        .to_title_case(),
                    name,
                    value,
                })
                .collect::<Vec<_>>();
            Self(stats)
        }
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct Player {
        pub id: PlayerId,
        pub characters: Vec<Character>,
        pub first_join_time: String,
        pub last_join_time: String,
        pub last_name: String,
        pub server_muted: bool,
        pub stats: PlayerStats,
    }

    impl From<(VeinPlayer, Vec<(String, VeinCharacter)>)> for Player {
        fn from((player, characters): (VeinPlayer, Vec<(String, VeinCharacter)>)) -> Self {
            let VeinPlayer {
                id,
                character_ids: _,
                first_join_time,
                last_join_time,
                last_name,
                b_server_muted,
                stats,
            } = player;

            Self {
                id: PlayerId::new(id),
                characters: characters.into_iter().map(Into::into).collect(),
                first_join_time,
                last_join_time,
                last_name,
                server_muted: b_server_muted,
                stats: PlayerStats::from(stats),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct PlayerStats {
        pub start_time: String,
        pub stats: Vec<PlayerStat>,
    }

    impl From<VeinPlayerStats> for PlayerStats {
        fn from(value: VeinPlayerStats) -> Self {
            let VeinPlayerStats {
                start_time,
                stats_keys,
                stats_values,
            } = value;
            Self {
                start_time,
                stats: stats_keys
                    .into_iter()
                    .zip(stats_values)
                    .map(|(name, value)| PlayerStat {
                        human_name: name.to_title_case(),
                        name,
                        value,
                    })
                    .collect(),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct PlayerStat {
        pub name: String,
        pub human_name: String,
        pub value: f64,
    }
}

pub mod agent {
    use crate::{
        config::Config,
        players::event::Player,
        utils::{Client, ServerEvent, ServerEventBus},
    };

    pub fn start(client: Client, event_bus: &ServerEventBus, config: &Config) {
        let refresh_interval = config.refresh_interval();
        let tx = event_bus.sender();

        tokio::spawn(async move {
            let mut timeout =
                tokio::time::interval(tokio::time::Duration::from_secs(refresh_interval));
            loop {
                timeout.tick().await;
                let Ok(vein_players) = client
                    .fetch_players()
                    .await
                    .inspect_err(|e| tracing::warn!("{e}"))
                else {
                    continue;
                };
                let mut players = Vec::with_capacity(vein_players.players.len());
                for vein_player_id in vein_players.players {
                    let Ok(player) = client
                        .fetch_player(&vein_player_id)
                        .await
                        .inspect_err(|e| tracing::warn!("{e}"))
                    else {
                        continue;
                    };

                    let characters = futures::future::join_all(
                        player
                            .character_ids
                            .clone()
                            .into_iter()
                            .map(|id| (id, client.clone()))
                            .map(|(id, client)| async move {
                                client
                                    .fetch_character(&id)
                                    .await
                                    .map(|character| (id, character))
                            }),
                    )
                    .await
                    .into_iter()
                    .filter_map(|res| res.inspect_err(|e| tracing::warn!("{e}")).ok())
                    .collect::<Vec<_>>();

                    players.push(Player::from((player, characters)));
                }
                if tx
                    .send(ServerEvent::Players(players))
                    .inspect_err(|e| tracing::error!("{e}"))
                    .is_err()
                {
                    break;
                }
            }
        });
    }
}
