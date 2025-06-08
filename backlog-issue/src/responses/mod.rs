use crate::{Attachment, Comment, Issue};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CountIssueResponse {
    pub count: u32,
}

pub type GetIssueResponse = Issue;
pub type AddIssueResponse = Issue;
pub type DeleteIssueResponse = Issue;
pub type UpdateIssueResponse = Issue;
pub type GetIssueListResponse = Vec<Issue>;
pub type GetCommentListResponse = Vec<Comment>;
pub type GetAttachmentListResponse = Vec<Attachment>;
