use std::collections::HashMap;
use std::sync::Arc;
use derive_new::new;
use reqwest::Client;
use tokio::sync::Mutex;

#[derive(new, Debug, Clone, Default)]
pub struct AppState {
    pub reqwest_client: Client,
    pub dynu_api_key: String,
    pub sync_domain_names: Vec<String>,
    #[new(default)]
    pub managed_domain_ids: Arc<Mutex<HashMap<String, u64>>>,
    #[new(default)]
    pub record_ids: Arc<Mutex<HashMap<String, u64>>>,
}