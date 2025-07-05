mod star_api;
pub use star_api::StarApi;

#[cfg(feature = "writable")]
mod add_star;
#[cfg(feature = "writable")]
pub use add_star::{AddStarParams, StarTarget};
