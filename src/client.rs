use crate::error::Result;
use url::Url;

#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    agent: ureq::Agent,
    auth_token: Option<String>,
}

impl Client {
    /// Creates a new Backlog API client
    pub fn new(base_url: &str) -> Result<Self> {
        Ok(Self {
            base_url: Url::parse(base_url)?,
            agent: ureq::agent(),
            auth_token: None,
        })
    }

    /// Sets the authentication token for the client
    pub fn with_auth_token(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }

    /// Makes a GET request to the specified path
    pub fn get<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.base_url.join(path)?;
        let mut req = self.agent.get(url.as_str());

        if let Some(token) = &self.auth_token {
            req = req.header("Authorization", &format!("Bearer {}", token));
        }

        let response = req.call()?.body_mut().read_json::<T>()?;
        Ok(response)
    }
}
