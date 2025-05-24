use backlog_api_core::Result;
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
        let request = self.prepare_request(reqwest::Method::GET, path, params)?;
        self.execute_request(request).await
    }

    /// Makes a POST request to the specified path with query parameters
    #[cfg(feature = "writable")]
    pub async fn post<T, P>(&self, path: &str, params: &P) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize,
    {
        let request = self.prepare_request(reqwest::Method::POST, path, params)?;
        self.execute_request(request).await
    }

    /// Makes a DELETE request to the specified path
    pub async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let request = self.prepare_request(reqwest::Method::DELETE, path, &())?;
        self.execute_request(request).await
    }

    /// Makes a PATCH request to the specified path with form parameters
    #[cfg(feature = "writable")]
    pub async fn patch<T, P>(&self, path: &str, params: &P) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize,
    {
        let request = self.prepare_request(reqwest::Method::PATCH, path, params)?;
        self.execute_request(request).await
    }

    // Helper methods

    fn prepare_request<P: serde::Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        params: &P,
    ) -> Result<reqwest::Request> {
        let url = self.base_url.join(path)?;
        let mut builder = self.client.request(method.clone(), url);
        builder = builder.header("Accept", "application/json");
        builder = match method {
            reqwest::Method::GET => builder.query(params),
            reqwest::Method::POST | reqwest::Method::PATCH => builder.form(params),
            _ => builder,
        };

        if let Some(token) = &self.auth_token {
            builder = builder.bearer_auth(token);
        }

        let mut req = builder.build()?;

        if let Some(key) = &self.api_key {
            let url = req.url_mut();
            url.query_pairs_mut().append_pair("apiKey", key);
        }

        Ok(req)
    }

    async fn execute_request<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::Request,
    ) -> Result<T> {
        let response = self.client.execute(request).await?;
        let json = response.json::<serde_json::Value>().await?;
        let entity = serde_json::from_value(json)?;
        Ok(entity)
    }
}
