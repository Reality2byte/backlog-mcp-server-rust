// Re-export watching types from backlog-watching crate
// This maintains backward compatibility for existing users of backlog-user
pub use backlog_watching::{
    GetWatchingCountResponse, GetWatchingListResponse, Watching, WatchingCount, WatchingType,
};
