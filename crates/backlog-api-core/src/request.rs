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
/// # Implementation Strategy
///
/// Each request parameter struct should:
/// 1. Include all necessary path components (project_id, repo_name, etc.)
/// 2. Know its target HTTP method (GET, POST, PATCH, DELETE)
/// 3. Know its API endpoint path
/// 4. Handle its own parameter serialization
///
/// # Examples
///
/// ```no_run
/// use backlog_api_core::{IntoRequest, Result};
/// use backlog_core::{ProjectIdOrKey, RepositoryIdOrName, identifier::PullRequestNumber};
/// use reqwest::Client;
/// use url::Url;
///
/// struct AddPullRequestCommentParams {
///     pub project_id_or_key: ProjectIdOrKey,
///     pub repo_id_or_name: RepositoryIdOrName,
///     pub pr_number: PullRequestNumber,
///     pub content: String,
///     pub notified_user_ids: Option<Vec<u32>>,
/// }
///
/// impl IntoRequest for AddPullRequestCommentParams {
///     fn into_request(self, base_url: &Url, client: &ReqwestClient) -> Result<reqwest::Request> {
///         let path = format!(
///             "/api/v2/projects/{}/git/repositories/{}/pullRequests/{}/comments",
///             self.project_id_or_key, self.repo_id_or_name, self.pr_number.value()
///         );
///         let url = base_url.join(&path)?;
///         
///         let mut form = vec![("content".to_string(), self.content)];
///         if let Some(user_ids) = self.notified_user_ids {
///             for id in user_ids {
///                 form.push(("notifiedUserId[]".to_string(), id.to_string()));
///             }
///         }
///         
///         let request = client.post(url).form(&form).build()?;
///         Ok(request)
///     }
/// }
/// ```
pub trait IntoRequest {
    /// Converts the parameter into a complete HTTP request.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the API (e.g., "https://example.backlog.jp")
    /// * `client` - The reqwest client to use for building the request
    ///
    /// Returns a ready-to-execute reqwest::Request object.
    fn into_request(self, base_url: &Url, client: &ReqwestClient) -> Result<reqwest::Request>;

    fn post<T: Serialize + ?Sized>(
        &self,
        base_url: &Url,
        path: String,
        client: &ReqwestClient,
        form: &T,
    ) -> Result<reqwest::Request> {
        let url = base_url.join(&path)?;

        let request = client
            .post(url)
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
            fn into_request(
                self,
                base_url: &Url,
                client: &ReqwestClient,
            ) -> Result<reqwest::Request> {
                let url = base_url.join("/test")?;
                let request = client.get(url).build()?;
                Ok(request)
            }
        }

        // If this compiles, the trait definition is correct
    }
}
