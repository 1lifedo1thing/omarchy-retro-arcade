//! FreeSki's production simulation is independent of the native frontend.
#[cfg(feature = "ui")]
pub mod app;
pub mod collision;
pub mod engine;
#[cfg(feature = "ui")]
mod input;
#[cfg(feature = "ui")]
mod render;
pub mod storage;
pub mod world;
