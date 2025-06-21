use crate::Result;
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
    /// Converts the parameter into a complete HTTP request.
    ///
    /// # Arguments
    /// * `client` - The reqwest client to use for building the request
    /// * `base_url` - The base URL for the API (e.g., "https://example.backlog.jp")
    ///
    /// Returns a ready-to-execute reqwest::Request object.
    fn into_request(self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request>;

    fn path(&self) -> String;
}

pub trait GetRequest: IntoRequest {
    fn to_query(&self) -> impl Serialize {
        &()
    }

    fn get(&self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        let path = self.path();
        let url = base_url.join(&path)?;
        let query = self.to_query();

        let request = client
            .get(url)
            .header("Accept", "application/json")
            .query(&query)
            .build()?;

        Ok(request)
    }
}

pub trait PostRequest: IntoRequest {
    fn to_form(&self) -> Vec<(String, String)>;

    fn post(&self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        let path = self.path();
        let url = base_url.join(&path)?;
        let form = self.to_form();

        let request = client
            .post(url)
            .header("Accept", "application/json")
            .form(&form)
            .build()?;

        Ok(request)
    }
}

pub trait DeleteRequest: IntoRequest {
    fn to_query(&self) -> impl Serialize {
        &()
    }

    fn delete(&self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        let path = self.path();
        let url = base_url.join(&path)?;
        let query = self.to_query();

        let request = client
            .delete(url)
            .header("Accept", "application/json")
            .query(&query)
            .build()?;

        Ok(request)
    }
}

pub trait PatchRequest: IntoRequest {
    fn to_form(&self) -> Vec<(String, String)>;

    fn patch(&self, client: &ReqwestClient, base_url: &Url) -> Result<reqwest::Request> {
        let path = self.path();
        let url = base_url.join(&path)?;
        let form = self.to_form();

        let request = client
            .patch(url)
            .header("Accept", "application/json")
            .form(&form)
            .build()?;

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_into_request_trait_compiles() {
        // This is a compilation test to ensure the trait is defined correctly
        #[allow(dead_code)]
        struct TestParams;

        impl IntoRequest for TestParams {
            fn path(&self) -> String {
                "/test".to_string()
            }
            fn into_request(
                self,
                client: &ReqwestClient,
                base_url: &Url,
            ) -> Result<reqwest::Request> {
                let url = base_url.join("")?;
                let request = client.get(url).build()?;
                Ok(request)
            }
        }

        // If this compiles, the trait definition is correct
    }
}
