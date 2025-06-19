use crate::models::{Attachment, Comment, Issue};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CountIssueResponse {
    pub count: u32,
}

#[derive(Debug, Deserialize)]
pub struct CountCommentResponse {
    pub count: u32,
}

pub type GetIssueResponse = Issue;
pub type AddIssueResponse = Issue;
pub type DeleteIssueResponse = Issue;
pub type UpdateIssueResponse = Issue;
pub type GetIssueListResponse = Vec<Issue>;
pub type GetCommentListResponse = Vec<Comment>;
pub type GetCommentResponse = Comment;
pub type GetAttachmentListResponse = Vec<Attachment>;
pub type AddCommentResponse = Comment;
