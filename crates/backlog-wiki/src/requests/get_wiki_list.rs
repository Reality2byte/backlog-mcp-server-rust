use backlog_core::ProjectIdOrKey;
use derive_builder::Builder;

#[derive(Debug, Builder, Clone, PartialEq)]
#[builder(setter(strip_option))]
pub struct GetWikiListParams {
    #[builder(default, setter(into))]
    pub project_id_or_key: Option<ProjectIdOrKey>,
    #[builder(default, setter(into))]
    pub keyword: Option<String>,
}

impl From<GetWikiListParams> for Vec<(String, String)> {
    fn from(params: GetWikiListParams) -> Self {
        let mut query_params = Vec::new();

        if let Some(project_id_or_key) = params.project_id_or_key {
            query_params.push(("projectIdOrKey".to_string(), project_id_or_key.to_string()));
        }

        if let Some(keyword) = params.keyword {
            query_params.push(("keyword".to_string(), keyword));
        }

        query_params
    }
}
