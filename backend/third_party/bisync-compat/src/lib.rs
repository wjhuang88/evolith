#![no_std]

// gix-protocol 0.64 imports the original `bisync` package.
// bisync2 intentionally preserves that API. Keep this crate behavior-free:
// it only supplies the expected package identity and re-exports the maintained fork.
pub use bisync2::*;
