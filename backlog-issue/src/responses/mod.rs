use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CountIssueResponse {
    pub count: u32,
}
