use std::collections::HashMap;
use std::sync::Arc;
use derive_new::new;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(new, Debug, Clone, Default)]
pub struct AppState {
    pub reqwest_client: Client,
    pub dynu_api_key: String,
    pub sync_domain_names: Vec<String>,
    pub group_name: Option<String>,
    #[new(default)]
    pub managed_domain_ids: Arc<Mutex<HashMap<String, u64>>>,
    #[new(default)]
    pub record_ids: Arc<Mutex<HashMap<RecordHash, u64>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, new, Eq, PartialEq, Hash)]
pub struct RecordHash {
    pub hostname: String,
    pub record_type: String,
}