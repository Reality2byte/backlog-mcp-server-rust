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

    /// Check access permissions for the specified project ID
    pub fn check_project_access_by_id(&self, project_id: &ProjectId) -> Result<(), Error> {
        // If no allow list is set, allow access to all projects
        let Some(allowed) = &self.allowed_projects else {
            return Ok(());
        };

        // Check if it's included in the allow list
        if allowed.iter().any(|allowed_proj| {
            matches!(allowed_proj, ProjectIdOrKey::Id(allowed_id) if allowed_id == project_id)
        }) {
            Ok(())
        } else {
            Err(Error::ProjectAccessDenied {
                project: project_id.to_string(),
                allowed_projects: allowed.iter().map(|p| format!("{p}")).collect(),
            })
        }
    }

    /// Check access permissions for the specified project key
    pub fn check_project_access_by_key(&self, project_key: &ProjectKey) -> Result<(), Error> {
        // If no allow list is set, allow access to all projects
        let Some(allowed) = &self.allowed_projects else {
            return Ok(());
        };

        // Check if it's included in the allow list
        if allowed.iter().any(|allowed_proj| {
            matches!(allowed_proj, ProjectIdOrKey::Key(allowed_key) if allowed_key == project_key)
        }) {
            Ok(())
        } else {
            Err(Error::ProjectAccessDenied {
                project: project_key.to_string(),
                allowed_projects: allowed.iter().map(|p| format!("{p}")).collect(),
            })
        }
    }

    /// Check access permissions for the specified project (accepts ProjectIdOrKey)
    pub fn check_project_access_id_or_key(&self, project: &ProjectIdOrKey) -> Result<(), Error> {
        match project {
            ProjectIdOrKey::Id(id) => self.check_project_access_by_id(id),
            ProjectIdOrKey::Key(key) => self.check_project_access_by_key(key),
            ProjectIdOrKey::EitherIdOrKey(id, _) => self.check_project_access_by_id(id),
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
    use std::sync::Mutex;

    // Ensure tests run sequentially to avoid environment variable conflicts
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_access_control_disabled_by_default() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("BACKLOG_PROJECTS");
        }
        let access_control = AccessControl::new().unwrap();
        assert!(!access_control.is_enabled());
        // When disabled, any project is allowed
        let project_key = ProjectKey::from_str("ANY_PROJECT").unwrap();
        assert!(access_control.check_project_access_by_key(&project_key).is_ok());
    }

    #[test]
    fn test_access_control_with_project_keys() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_A,PROJECT_B,PROJECT_C");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());
        
        let project_a = ProjectKey::from_str("PROJECT_A").unwrap();
        assert!(access_control.check_project_access_by_key(&project_a).is_ok());
        
        let project_b = ProjectKey::from_str("PROJECT_B").unwrap();
        assert!(access_control.check_project_access_by_key(&project_b).is_ok());
        
        let project_c = ProjectKey::from_str("PROJECT_C").unwrap();
        assert!(access_control.check_project_access_by_key(&project_c).is_ok());
        
        let project_d = ProjectKey::from_str("PROJECT_D").unwrap();
        assert!(access_control.check_project_access_by_key(&project_d).is_err());
    }

    #[test]
    fn test_access_control_with_project_ids() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "123456,789012");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());
        
        let project_id_1 = ProjectId::new(123456);
        assert!(access_control.check_project_access_by_id(&project_id_1).is_ok());
        
        let project_id_2 = ProjectId::new(789012);
        assert!(access_control.check_project_access_by_id(&project_id_2).is_ok());
        
        let project_id_3 = ProjectId::new(999999);
        assert!(access_control.check_project_access_by_id(&project_id_3).is_err());
    }

    #[test]
    fn test_access_control_mixed_keys_and_ids() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_A, 123456, PROJECT_C");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());
        
        let project_a = ProjectKey::from_str("PROJECT_A").unwrap();
        assert!(access_control.check_project_access_by_key(&project_a).is_ok());
        
        let project_id = ProjectId::new(123456);
        assert!(access_control.check_project_access_by_id(&project_id).is_ok());
        
        let project_c = ProjectKey::from_str("PROJECT_C").unwrap();
        assert!(access_control.check_project_access_by_key(&project_c).is_ok());
        
        let project_b = ProjectKey::from_str("PROJECT_B").unwrap();
        assert!(access_control.check_project_access_by_key(&project_b).is_err());
        
        let project_id_err = ProjectId::new(999999);
        assert!(access_control.check_project_access_by_id(&project_id_err).is_err());
    }

    #[test]
    fn test_access_control_empty_string() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "");
        }
        let access_control = AccessControl::new().unwrap();
        assert!(!access_control.is_enabled());
    }

    #[test]
    fn test_access_control_whitespace_handling() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", " PROJECT_A , PROJECT_B , ");
        }
        let access_control = AccessControl::new().unwrap();

        assert!(access_control.is_enabled());
        
        let project_a = ProjectKey::from_str("PROJECT_A").unwrap();
        assert!(access_control.check_project_access_by_key(&project_a).is_ok());
        
        let project_b = ProjectKey::from_str("PROJECT_B").unwrap();
        assert!(access_control.check_project_access_by_key(&project_b).is_ok());
    }

    #[test]
    fn test_access_control_phase2_document_api() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "123456,PROJECT_X");
        }
        let access_control = AccessControl::new().unwrap();

        // Simulate checking access after retrieving document with project_id
        let project_id_1 = ProjectId::new(123456);
        assert!(access_control.check_project_access_by_id(&project_id_1).is_ok());
        
        let project_x = ProjectKey::from_str("PROJECT_X").unwrap();
        assert!(access_control.check_project_access_by_key(&project_x).is_ok());
        
        let project_id_err = ProjectId::new(999999);
        assert!(access_control.check_project_access_by_id(&project_id_err).is_err());
        
        let project_y = ProjectKey::from_str("PROJECT_Y").unwrap();
        assert!(access_control.check_project_access_by_key(&project_y).is_err());
    }

    #[test]
    fn test_access_control_phase2_wiki_api() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "WIKI_PROJ");
        }
        let access_control = AccessControl::new().unwrap();

        // Simulate checking access after retrieving wiki with project_id
        let wiki_proj = ProjectKey::from_str("WIKI_PROJ").unwrap();
        assert!(access_control.check_project_access_by_key(&wiki_proj).is_ok());
        
        let other_proj = ProjectKey::from_str("OTHER_PROJ").unwrap();
        assert!(access_control.check_project_access_by_key(&other_proj).is_err());

        // Check error message contains allowed projects
        let err = access_control
            .check_project_access_by_key(&other_proj)
            .unwrap_err();
        match err {
            crate::error::Error::ProjectAccessDenied {
                project,
                allowed_projects,
            } => {
                assert_eq!(project, "OTHER_PROJ");
                assert_eq!(allowed_projects, vec!["WIKI_PROJ"]);
            }
            _ => panic!("Expected ProjectAccessDenied error"),
        }
    }

    #[test]
    fn test_access_control_phase3_issue_comment_api() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "ISSUE_PROJ,789012");
        }
        let access_control = AccessControl::new().unwrap();

        // Simulate checking access after retrieving issue with project_id
        let issue_proj = ProjectKey::from_str("ISSUE_PROJ").unwrap();
        assert!(access_control.check_project_access_by_key(&issue_proj).is_ok());
        
        let project_id = ProjectId::new(789012);
        assert!(access_control.check_project_access_by_id(&project_id).is_ok());
        
        let other_proj = ProjectKey::from_str("OTHER_PROJ").unwrap();
        assert!(access_control.check_project_access_by_key(&other_proj).is_err());
        
        let project_id_err = ProjectId::new(111111);
        assert!(access_control.check_project_access_by_id(&project_id_err).is_err());
    }

    #[test]
    fn test_check_project_access_by_id_with_allowed_id() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "123456,789012");
        }
        let access_control = AccessControl::new().unwrap();

        let project_id = ProjectId::new(123456);
        assert!(
            access_control
                .check_project_access_by_id(&project_id)
                .is_ok()
        );

        let project_id = ProjectId::new(789012);
        assert!(
            access_control
                .check_project_access_by_id(&project_id)
                .is_ok()
        );
    }

    #[test]
    fn test_check_project_access_by_id_with_denied_id() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "123456,789012");
        }
        let access_control = AccessControl::new().unwrap();

        let project_id = ProjectId::new(999999);
        let err = access_control
            .check_project_access_by_id(&project_id)
            .unwrap_err();
        match err {
            crate::error::Error::ProjectAccessDenied {
                project,
                allowed_projects,
            } => {
                assert_eq!(project, "999999");
                assert_eq!(allowed_projects, vec!["123456", "789012"]);
            }
            _ => panic!("Expected ProjectAccessDenied error"),
        }
    }

    #[test]
    fn test_check_project_access_by_id_when_disabled() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("BACKLOG_PROJECTS");
        }
        let access_control = AccessControl::new().unwrap();

        let project_id = ProjectId::new(123456);
        assert!(
            access_control
                .check_project_access_by_id(&project_id)
                .is_ok()
        );
    }

    #[test]
    fn test_check_project_access_by_key_with_allowed_key() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_A,PROJECT_B");
        }
        let access_control = AccessControl::new().unwrap();

        let project_key = ProjectKey::from_str("PROJECT_A").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_key)
                .is_ok()
        );

        let project_key = ProjectKey::from_str("PROJECT_B").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_key)
                .is_ok()
        );
    }

    #[test]
    fn test_check_project_access_by_key_with_denied_key() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BACKLOG_PROJECTS", "PROJECT_A,PROJECT_B");
        }
        let access_control = AccessControl::new().unwrap();

        let project_key = ProjectKey::from_str("PROJECT_C").unwrap();
        let err = access_control
            .check_project_access_by_key(&project_key)
            .unwrap_err();
        match err {
            crate::error::Error::ProjectAccessDenied {
                project,
                allowed_projects,
            } => {
                assert_eq!(project, "PROJECT_C");
                assert_eq!(allowed_projects, vec!["PROJECT_A", "PROJECT_B"]);
            }
            _ => panic!("Expected ProjectAccessDenied error"),
        }
    }

    #[test]
    fn test_check_project_access_by_key_when_disabled() {
        let _lock = TEST_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("BACKLOG_PROJECTS");
        }
        let access_control = AccessControl::new().unwrap();

        let project_key = ProjectKey::from_str("ANY_PROJECT").unwrap();
        assert!(
            access_control
                .check_project_access_by_key(&project_key)
                .is_ok()
        );
    }
}
