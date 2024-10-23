mod dynu;
mod serde;

use crate::state::{AppState, RecordHash};
use axum::extract::State;
use axum::Json;
use axum_macros::debug_handler;
use derive_new::new;
use ::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::id;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use reqwest::Request;
use serde_json::Value;
use crate::http::dynu::{DnsResponse, RecordsResponse, RecordResponse, RecordRequest};

#[debug_handler]
pub async fn retrieve_dns_records(
    State(AppState { reqwest_client, dynu_api_key, sync_domain_names, managed_domain_ids, record_ids, .. }): State<AppState>,
) -> Json<Vec<Endpoint>> {

    tracing::debug!("GET to /records (retrieve_dns_records)");

    let mut endpoints = Vec::<Endpoint>::new();
    let response = reqwest::Client::new().get("https://api.dynu.com/v2/dns")
        .header("Accept", "application/json")
        .header("API-Key", dynu_api_key.clone())
        .send().await;

    let Ok(response) = response else {
        panic!()
    };

    let text = response.text().await.unwrap();
    let dns_response = serde_json::from_str::<DnsResponse>(text.as_str()).unwrap();
    let mut domain_name_id_correlation_map = HashMap::<String, u64>::new();

    for domain in dns_response.domains {
        if sync_domain_names.contains(&domain.name) {
            domain_name_id_correlation_map.insert(domain.name, domain.id);
            let response = reqwest::Client::new().get(format!("https://api.dynu.com/v2/dns/{}/record", domain.id))
                .header("Accept", "application/json")
                .header("API-Key", dynu_api_key.clone())
                .send().await.unwrap();
            let records_response = serde_json::from_str::<RecordsResponse>(response.text().await.unwrap().as_str()).unwrap();
            for record in records_response.dns_records {
                record_ids.lock().await.insert(RecordHash::new(record.hostname.clone(), record.record_type.clone()), record.id);
                let mut targets = Vec::<String>::new();
                if let Some(target) = record.ipv4_address { targets.push(target) }
                if let Some(target) = record.text_data { targets.push(target) }
                endpoints.push(Endpoint::new(
                    record.hostname,
                    targets,
                    record.record_type,
                    None,
                    Some(record.ttl as i64),
                    HashMap::new(),
                    None
                ));
            }
        }
    }

    managed_domain_ids.lock().await.clear();
    *managed_domain_ids.lock().await = domain_name_id_correlation_map;

    tracing::trace!("GET to /records returns {:?}", endpoints.clone());
    Json(endpoints)
}

#[debug_handler]
pub async fn apply_changes(
    State(AppState {
              reqwest_client,
              dynu_api_key,
              sync_domain_names,
              group_name,
              managed_domain_ids,
              record_ids
          }): State<AppState>,
    Json(apply_changes): Json<ApplyChanges>,
) -> impl IntoResponse {

    tracing::debug!("POST to /records (apply_changes) with {:?}", apply_changes);

    let managed_domain_ids = managed_domain_ids.lock().await.clone();
    tracing::debug!("now creating endpoints");
    for endpoint in apply_changes.clone().create {
        tracing::trace!("for endpoint {:?}...", &endpoint);
        dbg!(&managed_domain_ids);
        let Some((domain, id)) = managed_domain_ids.iter().find(|&(d, _)| endpoint.dns_name.ends_with(d.as_str())) else {
            continue
        };
        tracing::trace!("...found domain {:?} and domain id {:?}", domain, id);
        let record_request = RecordRequest::new(
            endpoint.dns_name.clone().strip_suffix(format!(".{}", domain).as_str()).unwrap().to_owned(),
            endpoint.record_type.clone(),
            300,
            true,
            group_name.clone(),
            match group_name {
                Some(_) => None,
                None => match endpoint.record_type.as_str() {
                    "A" => Some(endpoint.targets.first().unwrap().clone()),
                    _ => None,
                }
            },
            match endpoint.record_type.as_str() {
                "TXT" => Some(endpoint.targets.first().unwrap().clone()),
                _ => None,
            },
        );
        tracing::trace!("Creating record {:?}", &record_request);
        let response = reqwest::Client::new().post(format!("https://api.dynu.com/v2/dns/{}/record", id))
            .header("Accept", "application/json")
            .header("API-Key", dynu_api_key.clone())
            .json(&record_request)
            .send().await;
        tracing::trace!("Dynu response: {:?}", response)
    }
    tracing::debug!("now deleting endpoints");
    for endpoint in apply_changes.clone().delete {

        tracing::trace!("for endpoint {:?}...", &endpoint);
        let record_id = record_ids.lock().await.get(&RecordHash::new(endpoint.dns_name.clone(), endpoint.record_type.clone())).cloned();
        dbg!(record_id);
        let Some((domain, domain_id)) = managed_domain_ids.iter().find(|&(d, _)| endpoint.dns_name.ends_with(d.as_str())) else {
            continue
        };
        let Some(record_id) = record_id else { continue };
        tracing::trace!("Deleting record {:?}", &endpoint.dns_name);
        let response = reqwest::Client::new().delete(format!("https://api.dynu.com/v2/dns/{}/record/{}", domain_id, record_id))
            .header("Accept", "application/json")
            .header("API-Key", dynu_api_key.clone())
            .send().await;
        tracing::trace!("Dynu response: {:?}", response)
    }
    //dbg!(payload);
    tracing::trace!("POST to /records returns 200 (not correct, should be 204)");

    StatusCode::NO_CONTENT
}

#[debug_handler]
pub async fn retrieve_domain_filter(
    State(AppState { .. }): State<AppState>,
) -> impl IntoResponse {

    tracing::debug!("GET to / (retrieve_domain_filter)");

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/external.dns.webhook+json;version=1".parse().unwrap());
    let domain_filter = DomainFilter::new(Some(vec![".mikiloz.es".to_owned()]), None, None, None);

    tracing::trace!("GET to / returns {:?}", domain_filter.clone());
    (
        headers,
        Json(domain_filter),
    )
}

#[debug_handler]
pub async fn adjust_endpoints(
    State(AppState { .. }): State<AppState>,
    Json(endpoints): Json<Vec<Endpoint>>,
) -> impl IntoResponse {

    tracing::debug!("POST to /adjustendpoints (adjust_endpoints) with {:?}", endpoints);
    //dbg!(&endpoints);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/external.dns.webhook+json;version=1".parse().unwrap());
    headers.insert("Vary", "Content-Type".parse().unwrap());

    tracing::trace!("POST to /adjustendpoints returns {:?}", endpoints.clone());
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

#[derive(Debug, Clone, new, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApplyChanges {
    pub create: Vec<Endpoint>,
    pub update_old: Vec<Endpoint>,
    pub update_new: Vec<Endpoint>,
    pub delete: Vec<Endpoint>,
}