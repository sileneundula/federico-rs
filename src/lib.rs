//! Fedi-Resort-Rust (Federated Resource)
//! 
//! ActivityPub

/// # \[ASTYPE::1] Actor (Social)
mod actor;

/// # \[ASTYPE::0] Activity
mod activity;

mod constants;

/// # \[AS::2] Media/Object Types
mod media;

mod target;

mod properties;

mod activitystreams;

/// Implements Common Traits For ActivityPub/ActivityStreams
pub mod traits;

/// Server-To-Server
mod s2s;
/// Client-To-Server
mod c2s;