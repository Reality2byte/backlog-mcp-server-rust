use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type UpdateStatusOrderResponse = Vec<backlog_domain_models::Status>;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateStatusOrderParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub status_ids: Vec<backlog_core::identifier::StatusId>,
}

#[cfg(feature = "writable")]
impl UpdateStatusOrderParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        status_ids: Vec<backlog_core::identifier::StatusId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            status_ids,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateStatusOrderParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/statuses/updateDisplayOrder",
            self.project_id_or_key
        )
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}

#[cfg(feature = "writable")]
impl From<&UpdateStatusOrderParams> for Vec<(String, String)> {
    fn from(params: &UpdateStatusOrderParams) -> Self {
        let mut seq = Vec::new();
        for status_id in &params.status_ids {
            seq.push(("statusId[]".to_string(), status_id.to_string()));
        }
        seq
    }
}
