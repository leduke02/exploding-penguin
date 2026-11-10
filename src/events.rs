//! Part 4: Events, Mountains & Endings.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.
//!
//! On a 3-person team, nobody owns this part. Leave this file as it is: it is the
//! placeholder module, it is ungraded, and the rest of the game builds and runs against it.

use crate::types::{EventKind, GameOver, GameState};

/// Rolls a random percentage and returns the matching event from the event table.
///
/// Skeleton returns `EventKind::HelpfulIndians`.
pub fn roll_random_event() -> EventKind {
    EventKind::HelpfulIndians
}

/// Applies one event's message and losses. Returns `Err` if the event ends the game.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn apply_event(state: &mut GameState, event: EventKind) -> Result<(), GameOver> {
    Ok(())
}

/// Mountain crossing past mile 950: rugged mountains, South Pass, Blue Mountains.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn cross_mountains(state: &mut GameState) -> Result<(), GameOver> {
    Ok(())
}

/// Rolls illness severity. Returns `Err(GameOver::RanOutOfMedicalSupplies)` when the
/// medicine runs out.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn illness_check(state: &mut GameState) -> Result<(), GameOver> {
    Ok(())
}

/// Prints the ending for a cause of death, then the formalities sequence.
///
/// Skeleton prints one generic line so a finished game is visible.
pub fn resolve_ending(state: &GameState, cause: GameOver) {
    println!("THE TRIP HAS ENDED: {:?}", cause);
}

/// Prints the arrival sequence: partial-turn food and date, final resource table.
///
/// Skeleton prints one generic line so a finished game is visible.
pub fn arrive_at_oregon_city(state: &mut GameState) {
    println!("YOU ARRIVED AT OREGON CITY");
}
