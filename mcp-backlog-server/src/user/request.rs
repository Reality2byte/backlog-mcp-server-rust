use rmcp::schemars::{self, JsonSchema};
use serde::Deserialize;

/// Request parameters for getting a list of users.
/// Currently, the Backlog API for listing users does not support any query parameters.
#[derive(Deserialize, JsonSchema, Debug, Default)]
pub struct GetUserListRequest {
    // No parameters are currently supported by the API endpoint /api/v2/users
    // Add fields here if API supports them in the future (e.g., keyword, count, order).
}
