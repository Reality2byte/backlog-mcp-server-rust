use backlog_api_core::Error;
use backlog_core::identifier::{Identifier, SharedFileId};
use derive_builder::Builder;

/// Parameters for linking shared files to an issue
#[cfg(feature = "writable")]
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into), build_fn(error = "Error"))]
pub struct LinkSharedFilesToIssueParams {
    /// List of shared file IDs to link to the issue
    pub shared_file_ids: Vec<SharedFileId>,
}

#[cfg(feature = "writable")]
impl From<&LinkSharedFilesToIssueParams> for Vec<(String, String)> {
    fn from(params: &LinkSharedFilesToIssueParams) -> Self {
        let mut seq = Vec::new();

        // Add each shared file ID with the array notation
        for file_id in &params.shared_file_ids {
            seq.push(("fileId[]".to_string(), file_id.value().to_string()));
        }

        seq
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;

    #[test]
    fn test_link_shared_files_to_issue_params_serialization() {
        let params = LinkSharedFilesToIssueParams {
            shared_file_ids: vec![SharedFileId::new(123), SharedFileId::new(456)],
        };

        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 2);
        assert_eq!(form_data[0], ("fileId[]".to_string(), "123".to_string()));
        assert_eq!(form_data[1], ("fileId[]".to_string(), "456".to_string()));
    }

    #[test]
    fn test_link_shared_files_to_issue_params_single_file() {
        let params = LinkSharedFilesToIssueParams {
            shared_file_ids: vec![SharedFileId::new(789)],
        };

        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 1);
        assert_eq!(form_data[0], ("fileId[]".to_string(), "789".to_string()));
    }

    #[test]
    fn test_link_shared_files_to_issue_params_builder() {
        let params = LinkSharedFilesToIssueParamsBuilder::default()
            .shared_file_ids(vec![SharedFileId::new(100), SharedFileId::new(200)])
            .build()
            .expect("Failed to build params");

        assert_eq!(params.shared_file_ids.len(), 2);
        assert_eq!(params.shared_file_ids[0].value(), 100);
        assert_eq!(params.shared_file_ids[1].value(), 200);
    }
}
