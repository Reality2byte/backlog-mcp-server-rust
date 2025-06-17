use backlog_core::{ProjectKey, TextFormattingRule, identifier::ProjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: ProjectId,
    pub project_key: ProjectKey,
    pub name: String,
    pub chart_enabled: bool,
    pub subtasking_enabled: bool,
    pub project_leader_can_edit_project_leader: bool,
    pub use_wiki: bool,
    pub use_file_sharing: bool,
    pub use_wiki_tree_view: bool,
    pub use_original_image_size_at_wiki: bool,
    pub text_formatting_rule: TextFormattingRule,
    pub archived: bool,
    pub display_order: i32,
    pub use_dev_attributes: bool,
}