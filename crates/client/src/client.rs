use backlog_api_core::{BacklogApiErrorResponse, Error as ApiError, IntoRequest, Result, bytes};
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use url::Url;

/// A type that represents a downloaded file's metadata and content.
#[derive(Debug, Clone)]
pub struct DownloadedFile {
    pub filename: String,
    pub content_type: String,
    pub bytes: bytes::Bytes,
}

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

    /// Executes a request using the IntoRequest trait
    pub async fn execute<T, P>(&self, params: &P) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        P: IntoRequest,
    {
        let mut request = params.to_request(&self.client, &self.base_url)?;

        // Add authentication headers to the request
        if let Some(token) = &self.auth_token {
            let headers = request.headers_mut();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", token).parse().map_err(|e| {
                    ApiError::InvalidBuildParameter(format!("Invalid auth token: {}", e))
                })?,
            );
        }

        if let Some(key) = &self.api_key {
            let url = request.url_mut();
            url.query_pairs_mut().append_pair("apiKey", key);
        }

        self.execute_request(request).await
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

    /// Makes a DELETE request to the specified path with form parameters
    #[cfg(feature = "writable")]
    pub async fn delete_with_params<T, P>(&self, path: &str, params: P) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize,
    {
        let request = self.prepare_request(reqwest::Method::DELETE, path, &params)?;
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

    /// Legacy method for backward compatibility - converts old IntoRequest to form data
    /// TODO: Remove this once all APIs are migrated to the new IntoRequest pattern
    #[cfg(feature = "writable")]
    pub async fn post_with_request<T, P>(&self, path: &str, params: P) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize,
    {
        let request = self.prepare_request(reqwest::Method::POST, path, &params)?;
        self.execute_request(request).await
    }

    /// Legacy method for backward compatibility - converts old IntoRequest to form data  
    /// TODO: Remove this once all APIs are migrated to the new IntoRequest pattern
    #[cfg(feature = "writable")]
    pub async fn patch_with_request<T, P>(&self, path: &str, params: P) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize,
    {
        let request = self.prepare_request(reqwest::Method::PATCH, path, &params)?;
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
            reqwest::Method::POST | reqwest::Method::PATCH | reqwest::Method::DELETE => {
                builder.form(params)
            }
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
        let response = self.client.execute(request).await?; // Returns ApiError::Http on reqwest error

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_body_text = response
                .text()
                .await
                .unwrap_or_else(|e| format!("Failed to read error body: {}", e));

            // Attempt to parse as BacklogApiErrorResponse
            match serde_json::from_str::<BacklogApiErrorResponse>(&error_body_text) {
                Ok(parsed_errors) => {
                    let summary = parsed_errors
                        .errors
                        .iter()
                        .map(|e| e.message.clone())
                        .collect::<Vec<String>>()
                        .join("; ");
                    return Err(ApiError::HttpStatus {
                        status,
                        errors: parsed_errors.errors,
                        errors_summary: summary,
                    });
                }
                Err(_) => {
                    // If parsing specific error structure fails, return a more generic error
                    // including the status and raw body.
                    // For now, we'll use the existing ApiError::Http, but ideally,
                    // we'd have a variant like HttpErrorWithBody or enhance ApiError::Http.
                    // This part needs careful consideration of how reqwest::Error is built for status errors.
                    // For simplicity, we'll re-create a basic error string.
                    // A better approach would be to ensure reqwest::Error::from(response) captures this.
                    // However, reqwest::Error::from_response is not public.
                    // We can construct a generic error message for now.
                    let summary = format!("HTTP Error {} with body: {}", status, error_body_text);
                    // This will be wrapped by ApiError::Http if we make a reqwest::Error
                    // For now, let's use a generic parameter error or a new dedicated variant if we add one.
                    // To keep it simple and avoid changing ApiError further in this step:
                    return Err(ApiError::InvalidBuildParameter(summary)); // Corrected to InvalidBuildParameter
                }
            }
        }

        // Success path
        let json_value = response.json::<serde_json::Value>().await?; // Can return ApiError::Http or ApiError::Json
        let entity = serde_json::from_value(json_value)?; // Can return ApiError::Json
        Ok(entity)
    }

    pub async fn download_file_raw(&self, path: &str) -> Result<DownloadedFile> {
        let request = self.prepare_request(reqwest::Method::GET, path, &())?;
        let response = self.client.execute(request).await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_body_text = response
                .text()
                .await
                .unwrap_or_else(|e| format!("Failed to read error body: {}", e));
            match serde_json::from_str::<BacklogApiErrorResponse>(&error_body_text) {
                Ok(parsed_errors) => {
                    let summary = parsed_errors
                        .errors
                        .iter()
                        .map(|e| e.message.clone())
                        .collect::<Vec<String>>()
                        .join("; ");
                    return Err(ApiError::HttpStatus {
                        status,
                        errors: parsed_errors.errors,
                        errors_summary: summary,
                    });
                }
                Err(_) => {
                    let summary = format!("HTTP Error {} with body: {}", status, error_body_text);
                    return Err(ApiError::InvalidBuildParameter(summary));
                }
            }
        }

        // Success path
        let headers = response.headers().clone();
        let bytes_content = response.bytes().await.map_err(ApiError::from)?;

        // Extract filename from Content-Disposition
        let filename = headers
            .get(CONTENT_DISPOSITION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| {
                // Simple parser for `filename="name.ext"` or `filename*=UTF-8''name.ext`
                if let Some(start) = value.find("filename=\"") {
                    let remainder = &value[start + 10..];
                    remainder.find('"').map(|end| remainder[..end].to_string())
                } else if let Some(start) = value.find("filename*=UTF-8''") {
                    let remainder = &value[start + 17..];
                    // This doesn't handle URL decoding, but it's a start
                    Some(remainder.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "downloaded_file".to_string()); // Default filename

        // Extract Content-Type
        let content_type = headers
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("application/octet-stream") // Default content type
            .to_string();

        Ok(DownloadedFile {
            filename,
            content_type,
            bytes: bytes_content,
        })
    }
}
