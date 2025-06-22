use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use derive_builder::Builder;
use serde::Serialize;

#[derive(Debug, Builder, Clone, PartialEq)]
#[builder(build_fn(error = "backlog_api_core::Error"), setter(strip_option))]
pub struct GetWikiCountParams {
    #[builder(default, setter(into))]
    pub project_id_or_key: Option<ProjectIdOrKey>,
}

impl From<GetWikiCountParams> for Vec<(String, String)> {
    fn from(params: GetWikiCountParams) -> Self {
        let mut query_params = Vec::new();

        if let Some(project_id_or_key) = params.project_id_or_key {
            query_params.push(("projectIdOrKey".to_string(), project_id_or_key.to_string()));
        }

        query_params
    }
}

impl IntoRequest for GetWikiCountParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/wikis/count".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.clone().into();
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::ProjectKey;

    #[test]
    fn test_get_wiki_count_params_with_project() {
        let project_key: ProjectKey = "TEST_PROJECT".parse().unwrap();
        let params = GetWikiCountParamsBuilder::default()
            .project_id_or_key(project_key)
            .build()
            .unwrap();

        let query_params: Vec<(String, String)> = params.into();
        assert_eq!(query_params.len(), 1);
        assert_eq!(query_params[0].0, "projectIdOrKey");
        assert_eq!(query_params[0].1, "TEST_PROJECT");
    }

    #[test]
    fn test_get_wiki_count_params_empty() {
        let params = GetWikiCountParamsBuilder::default().build().unwrap();

        let query_params: Vec<(String, String)> = params.into();
        assert!(query_params.is_empty());
    }
}
