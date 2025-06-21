use crate::Result;
use reqwest::{Client as ReqwestClient, Method};
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
    fn method(&self) -> Method;

    /// Returns the URL path for this request.
    fn path(&self) -> String;

    /// Returns query parameters for GET/DELETE requests.
    fn to_query(&self) -> impl Serialize {
        &()
    }

    /// Returns form data for POST/PATCH requests.
    fn to_form(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    /// Converts the parameter into a complete HTTP request.
    ///
    /// # Arguments
    /// * `client` - The reqwest client to use for building the request
    /// * `base_url` - The base URL for the API (e.g., "https://example.backlog.jp")
    ///
    /// Returns a ready-to-execute reqwest::Request object.
    fn to_request(&self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        let path = self.path();
        let url = base_url.join(&path)?;
        let method = self.method();

        let request_builder = client
            .request(method.clone(), url)
            .header("Accept", "application/json");

        let request = match method {
            Method::GET | Method::DELETE => {
                let query = IntoRequest::to_query(self);
                request_builder.query(&query).build()?
            }
            Method::POST | Method::PATCH => {
                let form = IntoRequest::to_form(self);
                request_builder.form(&form).build()?
            }
            _ => request_builder.build()?,
        };

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
            fn method(&self) -> Method {
                Method::GET
            }

            fn path(&self) -> String {
                "/test".to_string()
            }
        }

        // If this compiles, the trait definition is correct
    }
}
