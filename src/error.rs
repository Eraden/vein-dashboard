#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Fetch time: {0}")]
    FetchTime(reqwest::Error),
    #[error("Fetch weather: {0}")]
    FetchWeather(reqwest::Error),
    #[error("Fetch players: {0}")]
    FetchPlayers(reqwest::Error),
    #[error("Fetch player: {0}")]
    FetchPlayer(reqwest::Error),
    #[error("Fetch character: {0}")]
    FetchCharacter(reqwest::Error),
    #[error("Fetch status: {0}")]
    FetchStatus(reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
