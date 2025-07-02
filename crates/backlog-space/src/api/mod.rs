pub mod get_licence;
pub mod get_space;
pub mod get_space_disk_usage;
pub mod get_space_logo;
mod space_api;
#[cfg(feature = "writable")]
mod upload_attachment;

pub use space_api::SpaceApi;

pub use get_licence::{GetLicenceParams, GetLicenceResponse};
pub use get_space::{GetSpaceParams, GetSpaceResponse};
pub use get_space_disk_usage::{GetSpaceDiskUsageParams, GetSpaceDiskUsageResponse};
pub use get_space_logo::{GetSpaceLogoParams, GetSpaceLogoResponse};
#[cfg(feature = "writable")]
pub use upload_attachment::{UploadAttachmentParams, UploadAttachmentResponse};
