//! The Oregon Trail (MECC, 1978) ported from BASIC to Rust.
//!
//! This is the starter project. Each part lives in its own file so teammates rarely edit
//! the same file. The turn loop is not provided: your team writes it in `main` at
//! Checkpoint 3, when the four parts are integrated.

// The skeletons are not called from anywhere yet, and their parameters are unused.
// Remove these allows once your team has integrated the parts.
#![allow(dead_code, unused_variables)]

mod io;
mod setup;
mod fort_hunting;
mod travel;
mod events;
mod rng;
mod types;

fn main() {
    println!("THE OREGON TRAIL");
    println!("Starter project: write your part, then wire the parts together here.");
}
