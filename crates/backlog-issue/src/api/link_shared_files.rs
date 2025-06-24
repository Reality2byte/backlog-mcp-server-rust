#[cfg(feature = "writable")]
use crate::models::SharedFile;
#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
#[cfg(all(feature = "writable", feature = "macros"))]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::{IssueIdOrKey, identifier::SharedFileId};
#[cfg(feature = "writable")]
use derive_builder::Builder;
#[cfg(feature = "writable")]
use serde::Serialize;

/// Response type for linking shared files to an issue
#[cfg(feature = "writable")]
pub type LinkSharedFilesToIssueResponse = Vec<SharedFile>;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "macros", derive(ToFormParams))]
#[builder(build_fn(error = "ApiError"))]
pub struct LinkSharedFilesToIssueParams {
    #[builder(setter(into))]
    #[cfg_attr(feature = "macros", form(skip))]
    pub issue_id_or_key: IssueIdOrKey,
    #[builder(setter(into))]
    #[cfg_attr(feature = "macros", form(array, name = "fileId"))]
    pub shared_file_ids: Vec<SharedFileId>,
}

#[cfg(feature = "writable")]
impl IntoRequest for LinkSharedFilesToIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/issues/{}/sharedFiles", self.issue_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        #[cfg(feature = "macros")]
        {
            let params: Vec<(String, String)> = self.into();
            params
        }

        #[cfg(not(feature = "macros"))]
        {
            let mut params = Vec::new();

            for shared_file_id in &self.shared_file_ids {
                params.push(("fileId[]".to_string(), shared_file_id.to_string()));
            }

            params
        }
    }
}
