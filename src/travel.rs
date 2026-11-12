//! Part 3: Travel Decisions.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.

use crate::types::{EatingLevel, GameOver, GameState};
use crate::io::ask_number;

fn read_number() -> i32 {
    ask_number("") as i32
}

fn level_number(level: EatingLevel) -> i32 {
    match level {
        EatingLevel::Poor => 1,
        EatingLevel::Moderate => 2,
        EatingLevel::Well => 3,
    }
}

/// `n` must already be validated as 1..=3.
fn level_from_number(n: i32) -> EatingLevel {
    match n {
        1 => EatingLevel::Poor,
        2 => EatingLevel::Moderate,
        _ => EatingLevel::Well,
    }
}

/// poor = 13, moderate = 18, well = 23
fn food_cost(level: EatingLevel) -> f64 {
    8.0 + 5.0 * level_number(level) as f64
}
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
    loop {
        // cheapest level costs 13; below that nothing can ever be afforded
        if state.food < food_cost(EatingLevel::Poor) {
            return Err(GameOver::Starved);
        }

        // out-of-range input: silently show the same prompt again
        let n = loop {
            println!("DO YOU WANT TO EAT (1) POORLY (2) MODERATELY");
            println!("OR (3) WELL");
            let n = read_number();
            if (1..=3).contains(&n) {
                break n;
            }
        };

        match consume_food(state, level_from_number(n)) {
            Ok(()) => return Ok(level_from_number(n)),
            // unaffordable: restart from the very first prompt
            Err(()) => println!("YOU CAN'T EAT THAT WELL"),
        }
    }
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
