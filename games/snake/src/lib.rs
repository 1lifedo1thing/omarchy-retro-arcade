//! Versioned Snake simulation, usable by the shared replay service without a UI.
#[cfg(feature = "ui")]
pub mod app;
#[cfg(feature = "ui")]
mod audio;
pub mod engine;
pub mod replay;
#[cfg(feature = "ui")]
pub mod storage;
#[cfg(test)]
mod tests;
pub mod timing;
