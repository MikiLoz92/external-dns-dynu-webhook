use derive_new::new;
use reqwest::Client;

#[derive(new, Debug, Clone, Default)]
pub struct AppState {
    pub reqwest_client: Client,
    pub dynu_api_key: String,
    pub sync_domain_names: Vec<String>,
}