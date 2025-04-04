use core::panic;

use crate::error::Result;
use url::Url;

#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    client: reqwest::Client,
    auth_token: Option<String>,
    api_key: Option<String>,
}

impl Client {
    /// Creates a new Backlog API client
    pub fn new(base_url: &str) -> Result<Self> {
        Ok(Self {
            base_url: Url::parse(base_url)?,
            client: reqwest::Client::new(),
            auth_token: None,
            api_key: None,
        })
    }

    /// Sets the authentication token for the client
    pub fn with_auth_token(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Makes a GET request to the specified path
    pub async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.get_with_params(path, &()).await
    }

    /// Makes a GET request to the specified path with query parameters
    pub async fn get_with_params<T, P>(&self, path: &str, params: &P) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize,
    {
        let url = self.base_url.join(path)?;
        let mut req = self.client.get(url);

        req = req.query(params);

        if let Some(token) = &self.auth_token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let mut req = req.build()?;

        if let Some(key) = &self.api_key {
            let url = req.url_mut();
            url.query_pairs_mut().append_pair("apiKey", key);
        }

        let response = self.client.execute(req).await?;

        if let Ok(json) = response.json::<serde_json::Value>().await {
            println!("JSON parsed: {:?}", json);
            let entity: T = serde_json::from_value(json).unwrap();
            Ok(entity)
        } else {
            println!("No entity found in response");
            panic!("test");
        }
    }
}
