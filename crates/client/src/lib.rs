pub mod client;
pub use client::Client;
pub use client::DownloadedFile; // Re-export DownloadedFile

#[cfg(feature = "test-utils")]
pub mod test_utils;
