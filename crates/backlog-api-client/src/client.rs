use backlog_api_core::Result;
use client::Client;
use url::Url;

pub struct BacklogApiClient {
    client: Client,
}

impl BacklogApiClient {
    pub fn new(base_url: &str) -> Result<Self> {
        let url = Url::parse(base_url)?;
        let client = Client::new(url.as_ref())?;

        Ok(Self { client })
    }

    /// Sets the authentication token for the client
    pub fn with_auth_token(mut self, token: impl Into<String>) -> Self {
        self.client = self.client.with_auth_token(token.into());
        self
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.client = self.client.with_api_key(key.into());
        self
    }

    #[cfg(feature = "issue")]
    pub fn issue(&self) -> backlog_issue::IssueApi {
        backlog_issue::IssueApi::new(self.client.clone())
    }

    #[cfg(feature = "project")]
    pub fn project(&self) -> backlog_project::ProjectApi {
        backlog_project::ProjectApi::new(self.client.clone())
    }

    #[cfg(feature = "space")]
    pub fn space(&self) -> backlog_space::SpaceApi {
        backlog_space::SpaceApi::new(self.client.clone())
    }

    #[cfg(feature = "user")]
    pub fn user(&self) -> backlog_user::UserApi {
        backlog_user::UserApi::new(self.client.clone())
    }

    #[cfg(feature = "document")]
    pub fn document(&self) -> backlog_document::DocumentApi {
        backlog_document::DocumentApi::new(self.client.clone())
    }

    #[cfg(feature = "git")]
    pub fn git(&self) -> backlog_git::api::GitApi {
        backlog_git::api::GitApi::new(self.client.clone())
    }

    #[cfg(feature = "file")]
    pub fn file(&self) -> backlog_file::FileApi {
        backlog_file::FileApi::new(self.client.clone())
    }

    #[cfg(feature = "wiki")]
    pub fn wiki(&self) -> backlog_wiki::WikiApi {
        backlog_wiki::WikiApi::new(self.client.clone())
    }

    #[cfg(feature = "activity")]
    pub fn activity(&self) -> backlog_activity::ActivityApi {
        backlog_activity::ActivityApi::new(self.client.clone())
    }

    #[cfg(feature = "team")]
    pub fn team(&self) -> backlog_team::TeamApi {
        backlog_team::TeamApi::new(self.client.clone())
    }

    #[cfg(feature = "star")]
    pub fn star(&self) -> backlog_star::StarApi {
        backlog_star::StarApi::new(self.client.clone())
    }
}
