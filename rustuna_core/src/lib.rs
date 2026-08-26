//! Core components for Rustuna.
//!
//! This crate provides the central study, trial, storage, sampler, queue, and distribution
//! components used across Rustuna. Concrete storage backends, sampler implementations, and
//! language bindings are implemented in other crates in the workspace.

pub use error::{Error, ErrorKind};

pub mod attr;
pub mod datetime;
pub mod distribution;
pub mod multi_objective;
pub mod parzen_estimator;
pub mod sampler;
pub mod storage;
pub mod study;
pub mod transform;
pub mod trial;
pub mod trial_queue;

mod error;
mod string_interner;
mod study_cache;

// Not public API.
#[doc(hidden)]
pub mod __private {
    pub use crate::study_cache::StudyCache;
}

/// A crate-specific [`std::result::Result`] alias.
pub type Result<T, E = Error> = std::result::Result<T, E>;
