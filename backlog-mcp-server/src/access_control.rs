use crate::error::Error;
use backlog_core::identifier::ProjectId;
use backlog_core::{ProjectIdOrKey, ProjectKey};
use std::env;
use std::str::FromStr;

/// Structure to manage project access control
#[derive(Debug, Clone)]
pub struct AccessControl {
    /// List of allowed projects
    /// If None, all projects are accessible
    allowed_projects: Option<Vec<ProjectIdOrKey>>,
}

impl AccessControl {
    /// Initialize access control settings from environment variables
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let allowed_projects = env::var("BACKLOG_PROJECTS")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                s.split(',')
                    .map(|p| p.trim())
                    .filter(|p| !p.is_empty())
                    .map(|p| -> Result<ProjectIdOrKey, Box<dyn std::error::Error>> {
                        // Parse as either project ID or project key
                        if let Ok(id) = p.parse::<u32>() {
                            Ok(ProjectIdOrKey::Id(ProjectId::new(id)))
                        } else {
                            Ok(ProjectIdOrKey::Key(ProjectKey::from_str(p)?))
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        Ok(Self { allowed_projects })
    }

    /// Check access permissions for the specified project
    pub fn check_project_access(&self, project: &str) -> Result<(), Error> {
        // If no allow list is set, allow access to all projects
        let Some(allowed) = &self.allowed_projects else {
            return Ok(());
        };

        // Parse the input project identifier
        let target_project = if let Ok(id) = project.parse::<u32>() {
            ProjectIdOrKey::Id(ProjectId::new(id))
        } else {
            ProjectIdOrKey::Key(
                ProjectKey::from_str(project)
                    .map_err(|_| Error::Parameter(format!("Invalid project key: {project}")))?,
            )
        };

        // Check if it's included in the allow list
        if allowed.iter().any(|allowed_proj| {
            match (allowed_proj, &target_project) {
                // Compare IDs
                (ProjectIdOrKey::Id(allowed_id), ProjectIdOrKey::Id(target_id)) => {
                    allowed_id == target_id
                }
                // Compare keys
                (ProjectIdOrKey::Key(allowed_key), ProjectIdOrKey::Key(target_key)) => {
                    allowed_key == target_key
                }
                // Different formats are considered non-matching
                _ => false,
            }
        }) {
            Ok(())
        } else {
            Err(Error::ProjectAccessDenied {
                project: project.to_string(),
                allowed_projects: allowed.iter().map(|p| format!("{p}")).collect(),
            })
        }
    }

    /// Returns whether access control is enabled
    pub fn is_enabled(&self) -> bool {
        self.allowed_projects.is_some()
    }

    /// Get the list of allowed projects (for debugging)
    pub fn allowed_projects(&self) -> Option<&Vec<ProjectIdOrKey>> {
        self.allowed_projects.as_ref()
    }
}

impl Default for AccessControl {
    fn default() -> Self {
        Self::new().unwrap_or(Self {
            allowed_projects: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_access_control_disabled_by_default() {
        unsafe {
            env::remove_var("BACKLOG_PROJECTS");
        }
        let access_control = AccessControl::new().unwrap();
        assert!(!access_control.is_enabled());
        assert!(access_control.check_project_access("ANY_PROJECT").is_ok());
    }

    #[test]
    fn test_access_control_with_project_keys() {
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_A,PROJECT_B,PROJECT_C");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());
        assert!(access_control.check_project_access("PROJECT_A").is_ok());
        assert!(access_control.check_project_access("PROJECT_B").is_ok());
        assert!(access_control.check_project_access("PROJECT_C").is_ok());
        assert!(access_control.check_project_access("PROJECT_D").is_err());
    }

    #[test]
    fn test_access_control_with_project_ids() {
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "123456,789012");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());
        assert!(access_control.check_project_access("123456").is_ok());
        assert!(access_control.check_project_access("789012").is_ok());
        assert!(access_control.check_project_access("999999").is_err());
    }

    #[test]
    fn test_access_control_mixed_keys_and_ids() {
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_A, 123456, PROJECT_C");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());
        assert!(access_control.check_project_access("PROJECT_A").is_ok());
        assert!(access_control.check_project_access("123456").is_ok());
        assert!(access_control.check_project_access("PROJECT_C").is_ok());
        assert!(access_control.check_project_access("PROJECT_B").is_err());
        assert!(access_control.check_project_access("999999").is_err());
    }

    #[test]
    fn test_access_control_empty_string() {
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "");
        }
        let access_control = AccessControl::new().unwrap();
        assert!(!access_control.is_enabled());
    }

    #[test]
    fn test_access_control_whitespace_handling() {
        unsafe {
            env::set_var("BACKLOG_PROJECTS", " PROJECT_A , PROJECT_B , ");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());
        assert!(access_control.check_project_access("PROJECT_A").is_ok());
        assert!(access_control.check_project_access("PROJECT_B").is_ok());
    }
}
