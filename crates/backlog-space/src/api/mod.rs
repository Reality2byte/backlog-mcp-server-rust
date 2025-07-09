pub mod get_licence;
pub mod get_space;
pub mod get_space_disk_usage;
pub mod get_space_logo;
pub mod get_space_notification;
pub mod get_space_recent_updates;
mod space_api;
#[cfg(feature = "writable")]
mod update_space_notification;
#[cfg(feature = "writable")]
mod upload_attachment;

pub use space_api::SpaceApi;

pub use get_licence::{GetLicenceParams, GetLicenceResponse};
pub use get_space::{GetSpaceParams, GetSpaceResponse};
pub use get_space_disk_usage::{
    GetSpaceDiskUsageParams, GetSpaceDiskUsageResponse, ProjectDiskUsage,
};
pub use get_space_logo::{GetSpaceLogoParams, GetSpaceLogoResponse};
pub use get_space_notification::{GetSpaceNotificationParams, GetSpaceNotificationResponse};
pub use get_space_recent_updates::{GetSpaceRecentUpdatesParams, GetSpaceRecentUpdatesResponse};
#[cfg(feature = "writable")]
pub use update_space_notification::{
    UpdateSpaceNotificationParams, UpdateSpaceNotificationResponse,
};
#[cfg(feature = "writable")]
pub use upload_attachment::{UploadAttachmentParams, UploadAttachmentResponse};
