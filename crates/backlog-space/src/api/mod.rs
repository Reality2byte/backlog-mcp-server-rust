pub mod get_space;
pub mod get_space_logo;
pub mod get_space_notification;
mod space_api;
#[cfg(feature = "writable")]
mod upload_attachment;

pub use space_api::SpaceApi;

pub use get_space::{GetSpaceParams, GetSpaceResponse};
pub use get_space_logo::{GetSpaceLogoParams, GetSpaceLogoResponse};
pub use get_space_notification::{GetSpaceNotificationParams, GetSpaceNotificationResponse};
#[cfg(feature = "writable")]
pub use upload_attachment::{UploadAttachmentParams, UploadAttachmentResponse};
