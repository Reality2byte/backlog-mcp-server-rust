use backlog_api_core::Result;
use backlog_core::{User, identifier::UserId};
use client::Client;

pub struct UserApi(Client);

impl UserApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get the details of the authenticated user.
    pub async fn get_own_user(&self) -> Result<GetUserResponse> {
        self.0.get("/api/v2/users/myself").await
    }

    /// Get the list of users in the space.
    /// Corresponds to `GET /api/v2/users`.
    pub async fn get_user_list(&self) -> Result<Vec<User>> {
        self.0.get("/api/v2/users").await
    }

    /// Gets information about a specific user.
    ///
    /// Corresponds to `GET /api/v2/users/:userId`.
    pub async fn get_user(&self, user_id: impl Into<UserId>) -> Result<User> {
        let path = format!("/api/v2/users/{}", user_id.into());
        self.0.get(&path).await
    }

    /// Gets the user icon image data.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/icon`.
    pub async fn get_user_icon(&self, user_id: impl Into<UserId>) -> Result<Vec<u8>> {
        let path = format!("/api/v2/users/{}/icon", user_id.into());
        let downloaded_file = self.0.download_file_raw(&path).await?;
        Ok(downloaded_file.bytes.to_vec())
    }
}

type GetUserResponse = User;
// No specific response type needed for get_user_list as it returns Vec<User> directly.

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::Identifier;
    use client::test_utils::setup_client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_user_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = UserApi::new(client);
        let user_id = 123;

        let expected_user = serde_json::json!({
            "id": 123,
            "userId": "testuser",
            "name": "Test User",
            "roleType": 2,
            "lang": "ja",
            "mailAddress": "test@example.com",
            "lastLoginTime": "2024-06-20T06:35:39Z"
        });

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/users/{}", user_id)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let result = api.get_user(user_id).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id.value(), 123);
        assert_eq!(user.user_id, Some("testuser".to_string()));
        assert_eq!(user.name, "Test User");
        assert_eq!(user.mail_address, "test@example.com");
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = UserApi::new(client);
        let user_id = 999;

        Mock::given(method("GET"))
            .and(path(format!("/api/v2/users/{}", user_id)))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "errors": [{"message": "User not found"}]
            })))
            .mount(&mock_server)
            .await;

        let result = api.get_user(user_id).await;
        assert!(result.is_err());
    }
}
