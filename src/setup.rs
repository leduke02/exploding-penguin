//! Part 1: Foundation & Setup.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.

use crate::types::{GameOver, GameState};

/// Prints the skill menu, reads the choice with `io::ask_number`, and returns it.
/// A value above 5 silently becomes 0 (see the Part 1 page).
///
/// Skeleton returns 3.0 ("fair to middlin'").
pub fn get_shooting_skill_level() -> f64 {
    3.0
}

/// Runs the whole initial-purchase sequence and returns the starting `GameState`.
///
/// Skeleton returns a generic, plausible starting state so other parts can be tested.
pub fn purchase_supplies(skill_level: f64) -> GameState {
    GameState {
        food: 200.0,
        bullets: 1000.0,
        clothing: 50.0,
        misc_supplies: 50.0,
        cash: 100.0,
        oxen_spend: 250.0,
        skill_level,
        ..GameState::default()
    }
}

/// Advances to the next turn and prints its date. Returns `Err(GameOver::TrailTooLong)`
/// when the trip runs past the last date in the table.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn advance_turn_date(state: &mut GameState) -> Result<(), GameOver> {
    Ok(())
}

/// Start-of-turn cleanup and the status table: clamp negatives, low-food warning, floor
/// values, record `turn_start_mileage`, print mileage and the resource table.
///
/// Skeleton does nothing.
pub fn print_status_table(state: &mut GameState) {}

/// Charges the $20 doctor's bill when an illness or injury flag is set. Returns
/// `Err(GameOver::CantAffordDoctor)` when cash cannot cover it.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn apply_doctor_bill(state: &mut GameState) -> Result<(), GameOver> {
    Ok(())
}
