use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct AppConfig {
    pub base_url: String,
    pub api_key: String,
    #[serde(skip)]
    pub client: Arc<Client>, // Using Arc to easily share the client across threads
}
