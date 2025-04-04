use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueType {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub color: i32,
    pub display_order: i32,
}

impl IssueType {
    pub fn new(
        id: i32,
        project_id: i32,
        name: String,
        color: i32,
        display_order: i32,
    ) -> IssueType {
        IssueType {
            id,
            project_id,
            name,
            color,
            display_order,
        }
    }
}
