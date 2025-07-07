pub mod api;
pub mod models;

pub use api::WatchingApi;
pub use models::{
    GetWatchingCountResponse, GetWatchingListResponse, Watching, WatchingCount, WatchingType,
};

// Re-export API parameter types for convenience
pub use api::{GetWatchingParams, GetWatchingResponse};

#[cfg(feature = "writable")]
pub use api::{
    AddWatchingParams, AddWatchingResponse, DeleteWatchingParams, DeleteWatchingResponse,
    MarkAsReadParams, UpdateWatchingParams, UpdateWatchingResponse,
};
