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
        let url = self.base_url.join(path)?;
        let mut req = self.client.get(url);

        if let Some(token) = &self.auth_token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(key) = &self.api_key {
            req = req.query(&[("apiKey", key)]);
        }

        let response = req.send().await?;

        if let Ok(json) = response.json::<serde_json::Value>().await {
            println!("JSON parsed: {:?}", json);
            let entity: T = serde_json::from_value(json).unwrap();
            Ok(entity)
        } else {
            println!("No entity found in response");
            panic!("test");
        }
    }

    /// Makes a GET request to the specified path
    pub async fn get2<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.base_url.join(path)?;
        let mut req = self.client.get(url);

        if let Some(token) = &self.auth_token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(key) = &self.api_key {
            req = req.query(&[("apiKey", key)]);
        }

        let text = req.send().await.unwrap().text().await?;
        println!("Raw response body: {}", text);

        panic!("test");
    }
}
