use backlog_core::{User, identifier::ActivityId};
use backlog_domain_models::Project;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

pub type GetRecentUpdatesResponse = Vec<Activity>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: ActivityId,
    pub project: Project,
    #[serde(rename = "type")]
    pub type_id: i32,
    pub content: Content,
    pub notifications: Vec<Notification>,
    pub created_user: User,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Standard {
        id: i64,
        key_id: Option<i64>,
        summary: Option<String>,
        description: Option<String>,
        #[serde(default, deserialize_with = "deserialize_comment")]
        comment: Option<Comment>,
        changes: Option<Vec<Change>>,
    },
    UserManagement {
        users: Option<Vec<User>>,
        group_project_activities: Option<Vec<GroupProjectActivity>>,
        comment: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupProjectActivity {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Change {
    pub field: String,
    pub new_value: String,
    pub old_value: String,
    #[serde(rename = "type")]
    pub change_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    // Empty as per API spec example
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NulabAccount {
    pub nulab_id: String,
    pub name: String,
    pub unique_id: String,
}

fn deserialize_comment<'de, D>(deserializer: D) -> Result<Option<Comment>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct CommentVisitor;

    impl<'de> Visitor<'de> for CommentVisitor {
        type Value = Option<Comment>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("null, empty string, or Comment object")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value.is_empty() {
                Ok(None)
            } else {
                Err(E::custom("expected empty string or Comment object"))
            }
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let comment = Comment::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(Some(comment))
        }
    }

    deserializer.deserialize_any(CommentVisitor)
}
