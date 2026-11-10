//! Random numbers.
//!
//! The BASIC original calls `RND(-1)` everywhere. Every call site in this port goes
//! through [`rnd`], so there is exactly one place where randomness enters the program.

/// Returns a uniformly distributed number in `[0.0, 1.0)`, like BASIC's `RND`.
pub fn rnd() -> f64 {
    rand::random::<f64>()
}
