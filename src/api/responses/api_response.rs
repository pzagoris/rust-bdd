/// The Response from the API.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiResponse<T> {
    /// In caase of failure, this field contains the reason.
    pub error: Vec<String>,
    /// Data of response.
    pub result: Option<T>,
}
