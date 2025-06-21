/// Parameters for updating a wiki page.
///
/// This corresponds to the `PATCH /api/v2/wikis/:wikiId` endpoint.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateWikiParams {
    /// New page name (optional)
    pub name: Option<String>,
    /// New page content (optional)
    pub content: Option<String>,
    /// Whether to send email notification of update (optional)
    pub mail_notify: Option<bool>,
}

#[cfg(feature = "writable")]
impl UpdateWikiParams {
    /// Create a new UpdateWikiParams with all fields set to None
    pub fn new() -> Self {
        Self {
            name: None,
            content: None,
            mail_notify: None,
        }
    }

    /// Set the name field
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the content field
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set the mail_notify field
    pub fn mail_notify(mut self, mail_notify: bool) -> Self {
        self.mail_notify = Some(mail_notify);
        self
    }
}

#[cfg(feature = "writable")]
impl Default for UpdateWikiParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Form encoding for UpdateWikiParams
/// Converts parameters to form-encoded format for the Backlog API
#[cfg(feature = "writable")]
impl From<&UpdateWikiParams> for Vec<(String, String)> {
    fn from(params: &UpdateWikiParams) -> Self {
        let mut seq = Vec::new();

        if let Some(name) = &params.name {
            seq.push(("name".to_string(), name.clone()));
        }

        if let Some(content) = &params.content {
            seq.push(("content".to_string(), content.clone()));
        }

        if let Some(mail_notify) = params.mail_notify {
            seq.push(("mailNotify".to_string(), mail_notify.to_string()));
        }

        seq
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_params() {
        let params = UpdateWikiParams::new();

        assert!(params.name.is_none());
        assert!(params.content.is_none());
        assert!(params.mail_notify.is_none());
    }

    #[test]
    fn test_default_creates_empty_params() {
        let params = UpdateWikiParams::default();

        assert!(params.name.is_none());
        assert!(params.content.is_none());
        assert!(params.mail_notify.is_none());
    }

    #[test]
    fn test_builder_pattern() {
        let params = UpdateWikiParams::new()
            .name("Updated Wiki Title")
            .content("Updated wiki content")
            .mail_notify(true);

        assert_eq!(params.name, Some("Updated Wiki Title".to_string()));
        assert_eq!(params.content, Some("Updated wiki content".to_string()));
        assert_eq!(params.mail_notify, Some(true));
    }

    #[test]
    fn test_form_encoding_empty_params() {
        let params = UpdateWikiParams::new();
        let form_data: Vec<(String, String)> = (&params).into();

        assert!(form_data.is_empty());
    }

    #[test]
    fn test_form_encoding_name_only() {
        let params = UpdateWikiParams::new().name("Test Wiki");
        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 1);
        assert_eq!(form_data[0], ("name".to_string(), "Test Wiki".to_string()));
    }

    #[test]
    fn test_form_encoding_content_only() {
        let params = UpdateWikiParams::new().content("Test content");
        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 1);
        assert_eq!(
            form_data[0],
            ("content".to_string(), "Test content".to_string())
        );
    }

    #[test]
    fn test_form_encoding_mail_notify_only() {
        let params = UpdateWikiParams::new().mail_notify(true);
        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 1);
        assert_eq!(form_data[0], ("mailNotify".to_string(), "true".to_string()));
    }

    #[test]
    fn test_form_encoding_mail_notify_false() {
        let params = UpdateWikiParams::new().mail_notify(false);
        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 1);
        assert_eq!(
            form_data[0],
            ("mailNotify".to_string(), "false".to_string())
        );
    }

    #[test]
    fn test_form_encoding_all_params() {
        let params = UpdateWikiParams::new()
            .name("Updated Wiki Title")
            .content("Updated wiki content with\nmultiple lines")
            .mail_notify(true);

        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 3);

        // Check each parameter
        let name_param = form_data.iter().find(|(k, _)| k == "name");
        assert_eq!(
            name_param,
            Some(&("name".to_string(), "Updated Wiki Title".to_string()))
        );

        let content_param = form_data.iter().find(|(k, _)| k == "content");
        assert_eq!(
            content_param,
            Some(&(
                "content".to_string(),
                "Updated wiki content with\nmultiple lines".to_string()
            ))
        );

        let mail_notify_param = form_data.iter().find(|(k, _)| k == "mailNotify");
        assert_eq!(
            mail_notify_param,
            Some(&("mailNotify".to_string(), "true".to_string()))
        );
    }

    #[test]
    fn test_form_encoding_preserves_special_characters() {
        let params = UpdateWikiParams::new()
            .name("Test & Wiki <Title>")
            .content("Content with \"quotes\" and symbols: @#$%");

        let form_data: Vec<(String, String)> = (&params).into();

        assert_eq!(form_data.len(), 2);

        let name_param = form_data.iter().find(|(k, _)| k == "name");
        assert_eq!(
            name_param,
            Some(&("name".to_string(), "Test & Wiki <Title>".to_string()))
        );

        let content_param = form_data.iter().find(|(k, _)| k == "content");
        assert_eq!(
            content_param,
            Some(&(
                "content".to_string(),
                "Content with \"quotes\" and symbols: @#$%".to_string()
            ))
        );
    }
}
