use crate::{HttpMethod, Result};
use reqwest::Client as ReqwestClient;
use serde::Serialize;
use url::Url;

/// A trait for converting request parameters into a complete HTTP request.
///
/// This trait provides a unified interface for converting parameter types
/// into ready-to-execute reqwest::Request objects, including URL path construction,
/// HTTP method selection, and body serialization.
///
pub trait IntoRequest {
    /// Returns the HTTP method for this request.
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    /// Returns the URL path for this request.
    fn path(&self) -> String;

    /// Returns query parameters for GET/DELETE requests.
    fn to_query(&self) -> impl Serialize {
        &()
    }

    /// Returns form data for POST/PATCH requests.
    fn to_form(&self) -> impl Serialize {
        &()
    }

    /// Converts the parameter into a complete HTTP request.
    ///
    /// # Arguments
    /// * `client` - The reqwest client to use for building the request
    /// * `base_url` - The base URL for the API (e.g., "https://example.backlog.jp")
    ///
    /// Returns a ready-to-execute reqwest::Request object.
    fn into_request(self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request>
    where
        Self: Sized,
    {
        let path = self.path();
        let url = base_url.join(&path)?;
        let method = self.method();
        let reqwest_method = method.to_reqwest();

        let request_builder = client
            .request(reqwest_method, url)
            .header("Accept", "application/json");

        let request = match method {
            HttpMethod::Get | HttpMethod::Delete => {
                let query = IntoRequest::to_query(&self);
                request_builder.query(&query).build()?
            }
            HttpMethod::Post | HttpMethod::Patch => {
                let form = IntoRequest::to_form(&self);
                request_builder.form(&form).build()?
            }
        };

        Ok(request)
    }
}

/// A trait for converting request parameters into a download request.
///
/// Similar to IntoRequest but specifically designed for file download operations
/// that return binary data instead of JSON.
pub trait IntoDownloadRequest {
    /// Returns the URL path for this download request.
    fn path(&self) -> String;

    /// Converts the parameter into a complete HTTP GET request for downloading.
    ///
    /// # Arguments
    /// * `client` - The reqwest client to use for building the request
    /// * `base_url` - The base URL for the API (e.g., "https://example.backlog.jp")
    ///
    /// Returns a ready-to-execute reqwest::Request object for file download.
    fn into_request(self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request>
    where
        Self: Sized,
    {
        let path = self.path();
        let url = base_url.join(&path)?;

        let request = client.request(reqwest::Method::GET, url).build()?;

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_request_trait_compiles() {
        // This is a compilation test to ensure the trait is defined correctly
        #[allow(dead_code)]
        struct TestParams;

        impl IntoRequest for TestParams {
            fn path(&self) -> String {
                "/test".to_string()
            }
        }

        // If this compiles, the trait definition is correct
    }
}
