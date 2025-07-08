// Re-export types from backlog-domain-models with deprecation warnings
#[allow(deprecated)]
#[deprecated(
    since = "0.3.0",
    note = "Use backlog_domain_models::CustomFieldType instead"
)]
pub use backlog_domain_models::CustomFieldType;

#[allow(deprecated)]
#[deprecated(
    since = "0.3.0",
    note = "Use backlog_domain_models::CustomFieldSettings instead"
)]
pub use backlog_domain_models::CustomFieldSettings;

#[allow(deprecated)]
#[deprecated(
    since = "0.3.0",
    note = "Use backlog_domain_models::NumericSettings instead"
)]
pub use backlog_domain_models::NumericSettings;

#[allow(deprecated)]
#[deprecated(
    since = "0.3.0",
    note = "Use backlog_domain_models::DateSettings instead"
)]
pub use backlog_domain_models::DateSettings;

#[allow(deprecated)]
#[deprecated(
    since = "0.3.0",
    note = "Use backlog_domain_models::ListSettings instead"
)]
pub use backlog_domain_models::ListSettings;

// Re-export InitialDate and ListItem from backlog-domain-models
