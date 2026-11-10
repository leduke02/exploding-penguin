//! Part 3: Travel Decisions.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.

use crate::types::{EatingLevel, GameOver, GameState};

/// What the riders *look* like. The truth can differ (see `resolve_rider_encounter`).
/// This type belongs to Part 3 alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hostility {
    Hostile,
    Friendly,
}

/// Asks how well to eat, re-asking until the choice is valid and affordable. Returns the
/// level settled on. Returns `Err(GameOver::Starved)` when food is below 13.
///
/// Skeleton returns `Ok(EatingLevel::Moderate)` without prompting.
pub fn choose_eating_level(state: &mut GameState) -> Result<EatingLevel, GameOver> {
    Ok(EatingLevel::Moderate)
}

/// Deducts the food cost of an eating level if it can be afforded. `Err(())` means it
/// could not be afforded and nothing changed.
///
/// Skeleton returns `Ok(())` without changing anything.
pub fn consume_food(state: &mut GameState, level: EatingLevel) -> Result<(), ()> {
    Ok(())
}

/// Adds this turn's travel distance to `mileage`.
///
/// Skeleton does nothing.
pub fn calculate_daily_mileage(state: &mut GameState) {}

/// Rolls whether riders appear this turn. `None` means no encounter; otherwise returns
/// how the riders look.
///
/// Skeleton returns `None` (never an encounter).
pub fn rider_encounter_roll(mileage: f64) -> Option<Hostility> {
    None
}

/// Plays out the rider encounter: tactic menu, outcome, losses. Returns
/// `Err(GameOver::RiderMassacre)` if the bullets run out against hostile riders.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn resolve_rider_encounter(state: &mut GameState, looks: Hostility) -> Result<(), GameOver> {
    Ok(())
}
