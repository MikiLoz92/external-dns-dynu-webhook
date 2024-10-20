mod dynu;
mod serde;

use crate::state::AppState;
use axum::extract::State;
use axum::Json;
use axum_macros::debug_handler;
use derive_new::new;
use ::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use crate::http::dynu::DnsResponse;

#[debug_handler]
pub async fn retrieve_dns_records(
    State(AppState { reqwest_client, dynu_api_key, .. }): State<AppState>,
) -> Json<Vec<Endpoint>> {

    let response = reqwest_client.get("https://api.dynu.com/v2/dns")
        .header("Accept", "application/json")
        .header("API-Key", dynu_api_key)
        .send().await;

    let Ok(response) = response else {
        panic!()
    };

    let text = response.text().await.unwrap();
    dbg!(text.clone());
    let dns = serde_json::from_str::<DnsResponse>(text.as_str());
    dbg!(dns);

    Json(vec![])
}

#[debug_handler]
pub async fn retrieve_domain_filter(
    State(AppState { .. }): State<AppState>,
) -> impl IntoResponse {

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/external.dns.webhook+json;version=1".parse().unwrap());
    (
        headers,
        Json(DomainFilter::new(Some(vec![".mikiloz.es".to_owned()]), None, None, None)),
    )
}

#[debug_handler]
pub async fn adjust_endpoints(
    State(AppState { .. }): State<AppState>,
    Json(endpoints): Json<Vec<Endpoint>>,
) -> impl IntoResponse {

    dbg!(&endpoints);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/external.dns.webhook+json;version=1".parse().unwrap());
    headers.insert("Vary", "Content-Type".parse().unwrap());
    (
        headers,
        Json(endpoints),
    )
}


#[derive(Debug, Clone, new, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(rename = "dnsName")]
    pub dns_name: String,
    pub targets: Vec<String>,
    #[serde(rename = "recordType")]
    pub record_type: String,
    #[serde(rename = "setIdentifier", skip_serializing_if = "Option::is_none")]
    pub set_identifier: Option<String>,
    #[serde(rename = "recordTTL", skip_serializing_if = "Option::is_none")]
    pub record_ttl: Option<i64>,
    pub labels: HashMap<String, String>,
    #[serde(rename = "providerSpecific", skip_serializing_if = "Option::is_none")]
    pub provider_specific: Option<Vec<ProviderSpecificProperty>>,
}

#[derive(Debug, Clone, new, Serialize, Deserialize)]
pub struct ProviderSpecificProperty {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, new, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_include: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_exclude: Option<String>,
}