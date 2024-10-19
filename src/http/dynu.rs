use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsResponse {
    pub status_code: u16,
    pub domains: Vec<DomainResponse>,
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