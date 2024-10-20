use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsResponse {
    pub status_code: u16,
    pub domains: Vec<DomainResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynuHttpResponse<T> {
    pub status_code: u16,
    pub response: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainResponse {
    pub id: u64,
    pub name: String,
    pub unicode_name: String,
    pub token: String,
    pub state: String,
    pub group: Option<String>,
    pub ipv4_address: Option<String>,
    pub ipv6_address: Option<String>,
    pub ttl: u32,
    pub ipv4: bool,
    pub ipv6: bool,
    pub ipv4_wildcard_alias: bool,
    pub ipv6_wildcard_alias: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_zone_transfer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnssec: Option<bool>,
    pub created_on: String,
    pub updated_on: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordResponse {
    pub id: u64,
    pub domain_id: u64,
    pub domain_name: String,
    pub node_name: String,
    pub hostname: String,
    pub record_type: String,
    pub ttl: u32,
    pub state: bool,
    pub content: String,
    pub updated_on: String,
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}
