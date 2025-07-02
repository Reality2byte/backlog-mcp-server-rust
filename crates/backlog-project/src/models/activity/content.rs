use backlog_core::identifier::SvnRevision;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    IssueCreated(Box<IssueCreated>),
    Issue(Box<Issue>),
    IssueDeleted(Box<IssueDeleted>),
    IssueMultiUpdate(Box<IssueMultiUpdate>),
    Wiki(Box<Wiki>),
    File(Box<File>),
    Svn(Box<Svn>),
    Git(Box<Git>),
    GitRepositoryCreated(Box<GitRepositoryCreated>),
    EditMember(Box<EditMember>),
    PullRequest(Box<PullRequest>),
    Version(Box<Version>),
    VersionUpdated(Box<VersionUpdated>),
    ProjectTeam(Box<ProjectTeam>),
    StatusDeleted(Box<StatusDeleted>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueCreated {
    pub id: i32,
    pub key_id: i32,
    pub summary: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Issue {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueDeleted {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueMultiUpdate {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Wiki {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub id: i32,
    pub dir: String,
    pub name: String,
    pub size: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Svn {
    pub rev: SvnRevision,
    pub comment: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Git {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitRepositoryCreated {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditMember {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionUpdated {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectTeam {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusDeleted {}
