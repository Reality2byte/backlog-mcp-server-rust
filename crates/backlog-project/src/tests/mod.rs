use crate::models::Project;

pub type GetProjectListResponse = Vec<Project>;
pub type GetProjectResponse = Project;
pub type GetVersionMilestoneListResponse = Vec<backlog_domain_models::Milestone>;