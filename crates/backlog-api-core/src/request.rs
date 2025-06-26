use crate::{HttpMethod, Result};
use reqwest::Client as ReqwestClient;
use serde::Serialize;
use std::path::PathBuf;
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

/// A trait for converting request parameters into an upload request.
///
/// Similar to IntoRequest but specifically designed for file upload operations
/// that use multipart/form-data encoding.
pub trait IntoUploadRequest {
    /// Returns the URL path for this upload request.
    fn path(&self) -> String;

    /// Returns the file path to upload.
    fn file_path(&self) -> &PathBuf;

    /// Returns the form field name for the file (default: "file").
    fn file_field_name(&self) -> &str {
        "file"
    }

    /// Returns additional form fields to include in the multipart request.
    fn additional_fields(&self) -> Vec<(String, String)> {
        Vec::new()
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

    #[test]
    fn test_into_upload_request_trait_compiles() {
        use std::path::PathBuf;

        #[allow(dead_code)]
        struct TestUploadParams {
            file_path: PathBuf,
        }

        impl IntoUploadRequest for TestUploadParams {
            fn path(&self) -> String {
                "/test/upload".to_string()
            }

            fn file_path(&self) -> &PathBuf {
                &self.file_path
            }
        }

        // Test default implementations
        let params = TestUploadParams {
            file_path: PathBuf::from("/test/file.txt"),
        };

        assert_eq!(params.file_field_name(), "file");
        assert_eq!(params.additional_fields().len(), 0);
    }

    #[test]
    fn test_into_upload_request_with_custom_field_name() {
        use std::path::PathBuf;

        struct CustomUploadParams {
            file_path: PathBuf,
        }

        impl IntoUploadRequest for CustomUploadParams {
            fn path(&self) -> String {
                "/custom/upload".to_string()
            }

            fn file_path(&self) -> &PathBuf {
                &self.file_path
            }

            fn file_field_name(&self) -> &str {
                "custom_file"
            }
        }

        let params = CustomUploadParams {
            file_path: PathBuf::from("/test/custom.txt"),
        };

        assert_eq!(params.file_field_name(), "custom_file");
    }

    #[test]
    fn test_into_upload_request_with_additional_fields() {
        use std::path::PathBuf;

        struct UploadWithMetadata {
            file_path: PathBuf,
            description: String,
        }

        impl IntoUploadRequest for UploadWithMetadata {
            fn path(&self) -> String {
                "/metadata/upload".to_string()
            }

            fn file_path(&self) -> &PathBuf {
                &self.file_path
            }

            fn additional_fields(&self) -> Vec<(String, String)> {
                vec![
                    ("description".to_string(), self.description.clone()),
                    ("type".to_string(), "document".to_string()),
                ]
            }
        }

        let params = UploadWithMetadata {
            file_path: PathBuf::from("/test/doc.pdf"),
            description: "Test document".to_string(),
        };

        let fields = params.additional_fields();
        assert_eq!(fields.len(), 2);
        assert_eq!(
            fields[0],
            ("description".to_string(), "Test document".to_string())
        );
        assert_eq!(fields[1], ("type".to_string(), "document".to_string()));
    }
}
