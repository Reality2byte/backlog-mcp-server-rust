mod watching_api;
pub use watching_api::WatchingApi;

mod get_watching;
pub use get_watching::{GetWatchingParams, GetWatchingResponse};

#[cfg(feature = "writable")]
mod add_watching;
#[cfg(feature = "writable")]
pub use add_watching::{AddWatchingParams, AddWatchingResponse};

#[cfg(feature = "writable")]
mod update_watching;
#[cfg(feature = "writable")]
pub use update_watching::{UpdateWatchingParams, UpdateWatchingResponse};

#[cfg(feature = "writable")]
mod delete_watching;
#[cfg(feature = "writable")]
pub use delete_watching::{DeleteWatchingParams, DeleteWatchingResponse};

#[cfg(feature = "writable")]
mod mark_as_read;
#[cfg(feature = "writable")]
pub use mark_as_read::MarkAsReadParams;
