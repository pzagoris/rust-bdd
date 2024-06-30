/// The response from the Server Time
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerTime {
    pub unixtime: u64,
    pub rfc1123: String,
}
