use crate::requests::{GetOwnUserParams, GetUserIconParams, GetUserListParams, GetUserParams};
use backlog_api_core::{IntoRequest, Result};
use backlog_core::{User, identifier::UserId};
use client::{Client, DownloadedFile};

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

    // New IntoRequest-based methods

    /// Get the list of users in the space using IntoRequest pattern.
    /// Corresponds to `GET /api/v2/users`.
    pub async fn get_user_list_v2(&self, params: GetUserListParams) -> Result<Vec<User>> {
        self.0.execute(params).await
    }

    /// Gets information about a specific user using IntoRequest pattern.
    ///
    /// Corresponds to `GET /api/v2/users/:userId`.
    pub async fn get_user_v2(&self, params: GetUserParams) -> Result<User> {
        self.0.execute(params).await
    }

    /// Get the details of the authenticated user using IntoRequest pattern.
    pub async fn get_own_user_v2(&self, params: GetOwnUserParams) -> Result<User> {
        self.0.execute(params).await
    }

    /// Gets the user icon image data using IntoRequest pattern.
    ///
    /// Corresponds to `GET /api/v2/users/:userId/icon`.
    pub async fn get_user_icon_v2(&self, params: GetUserIconParams) -> Result<DownloadedFile> {
        let path = params.path();
        self.0.download_file_raw(&path).await
    }
}

type GetUserResponse = User;
// No specific response type needed for get_user_list as it returns Vec<User> directly.

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::{Identifier, UserId};
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

    // New IntoRequest-based tests

    #[tokio::test]
    async fn test_get_user_v2_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = UserApi::new(client);
        let user_id = UserId::new(123);

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
            .and(path("/api/v2/users/123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let params = GetUserParams::new(user_id);
        let result = api.get_user_v2(params).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id.value(), 123);
        assert_eq!(user.user_id, Some("testuser".to_string()));
        assert_eq!(user.name, "Test User");
        assert_eq!(user.mail_address, "test@example.com");
    }

    #[tokio::test]
    async fn test_get_user_list_v2_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = UserApi::new(client);

        let expected_users = serde_json::json!([
            {
                "id": 123,
                "userId": "testuser1",
                "name": "Test User 1",
                "roleType": 2,
                "lang": "ja",
                "mailAddress": "test1@example.com",
                "lastLoginTime": "2024-06-20T06:35:39Z"
            },
            {
                "id": 124,
                "userId": "testuser2",
                "name": "Test User 2",
                "roleType": 1,
                "lang": "en",
                "mailAddress": "test2@example.com",
                "lastLoginTime": "2024-06-21T06:35:39Z"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v2/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_users))
            .mount(&mock_server)
            .await;

        let params = GetUserListParams::new();
        let result = api.get_user_list_v2(params).await;
        assert!(result.is_ok());
        let users = result.unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].id.value(), 123);
        assert_eq!(users[1].id.value(), 124);
    }

    #[tokio::test]
    async fn test_get_own_user_v2_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = UserApi::new(client);

        let expected_user = serde_json::json!({
            "id": 123,
            "userId": "myself",
            "name": "My User",
            "roleType": 1,
            "lang": "en",
            "mailAddress": "myself@example.com",
            "lastLoginTime": "2024-06-20T06:35:39Z"
        });

        Mock::given(method("GET"))
            .and(path("/api/v2/users/myself"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let params = GetOwnUserParams::new();
        let result = api.get_own_user_v2(params).await;
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id.value(), 123);
        assert_eq!(user.user_id, Some("myself".to_string()));
        assert_eq!(user.name, "My User");
        assert_eq!(user.mail_address, "myself@example.com");
    }

    #[tokio::test]
    async fn test_get_user_icon_v2_success() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let api = UserApi::new(client);
        let user_id = UserId::new(123);

        let icon_data = b"fake_icon_data";
        Mock::given(method("GET"))
            .and(path("/api/v2/users/123/icon"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(icon_data.as_slice())
                    .insert_header("content-type", "image/png")
                    .insert_header("content-disposition", "attachment; filename=\"icon.png\""),
            )
            .mount(&mock_server)
            .await;

        let params = GetUserIconParams::new(user_id);
        let result = api.get_user_icon_v2(params).await;
        assert!(result.is_ok());
        let downloaded_file = result.unwrap();
        assert_eq!(downloaded_file.filename, "icon.png");
        assert_eq!(downloaded_file.content_type, "image/png");
        assert_eq!(downloaded_file.bytes.as_ref(), icon_data);
    }
}
