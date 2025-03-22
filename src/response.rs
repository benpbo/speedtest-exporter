use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Response {
    Result {
        timestamp: DateTime<Utc>,
        ping: Ping,
        download: Download,
        upload: Upload,
        isp: String,
        interface: Interface,
        server: Server,
        result: Result,
    },
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ping {
    pub jitter: f64,
    pub latency: f64,
    pub low: f64,
    pub high: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Download {
    pub bandwidth: i64,
    pub bytes: i64,
    pub elapsed: i64,
    pub latency: DownloadLatency,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadLatency {
    pub iqm: f64,
    pub low: f64,
    pub high: f64,
    pub jitter: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub bandwidth: i64,
    pub bytes: i64,
    pub elapsed: i64,
    pub latency: UploadLatency,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadLatency {
    pub iqm: f64,
    pub low: f64,
    pub high: f64,
    pub jitter: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    pub internal_ip: String,
    pub name: String,
    pub mac_addr: String,
    pub is_vpn: bool,
    pub external_ip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub id: i64,
    pub host: String,
    pub port: i64,
    pub name: String,
    pub location: String,
    pub country: String,
    pub ip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub id: String,
    pub url: String,
    pub persisted: bool,
}
