//! Part 4: Events, Mountains & Endings.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.
//!
//! On a 3-person team, nobody owns this part. Leave this file as it is: it is the
//! placeholder module, it is ungraded, and the rest of the game builds and runs against it.

use crate::types::{EatingLevel, EventKind, GameOver, GameState};
use rand::Rng;

/// Rolls a random percentage and returns the matching event from the event table.
pub fn roll_random_event() -> EventKind {
    let roll: i32 = rand::thread_rng().gen_range(0..100);
    event_from_roll(roll)
}

fn event_from_roll(roll: i32) -> EventKind {
    if roll < 6 {
        EventKind::WagonBreakdown      // 1
    } else if roll < 11 {
        EventKind::OxInjuredLeg        // 2
    } else if roll < 13 {
        EventKind::DaughterBrokeArm    // 3
    } else if roll < 15 {
        EventKind::OxWandersOff        // 4
    } else if roll < 17 {
        EventKind::SonLost             // 5
    } else if roll < 22 {
        EventKind::UnsafeWater         // 6
    } else if roll < 32 {
        EventKind::RainOrCold          // 7
    } else if roll < 35 {
        EventKind::Bandits             // 8
    } else if roll < 37 {
        EventKind::WagonFire           // 9
    } else if roll < 42 {
        EventKind::HeavyFog            // 10
    } else if roll < 44 {
        EventKind::Snakebite           // 11
    } else if roll < 54 {
        EventKind::RiverSwamped        // 12
    } else if roll < 64 {
        EventKind::WildAnimals         // 13
    } else if roll < 69 {
        EventKind::HailStorm           // 14
    } else if roll < 95 {
        EventKind::EatingIllnessRoll   // 15
    } else {
        EventKind::HelpfulIndians      // 16
    }
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
fn eating_number(level: EatingLevel) -> f64 {
    match level {
        EatingLevel::Poor => 1.0,
        EatingLevel::Moderate => 2.0,
        EatingLevel::Well => 3.0,
      }
}

pub fn illness_check(state: &mut GameState) -> Result<(), GameOver> {
 let mut rng = rand::thread_rng();
    let e = eating_number(state.last_eating_level);

    // Roll 1: mild?  1..100 < 10 + 35 * (e - 1)
    let mild_chance = 10.0 + 35.0 * (e - 1.0);
    if (rng.gen_range(1..=100) as f64) < mild_chance {
        println!("MILD ILLNESS---MEDICINE USED");
        state.mileage -= 5.0;
        state.misc_supplies -= 2.0;
    } else {
        // Roll 2: bad vs serious.  1..100 < 100 - 40 / 4^(e - 1)
        let bad_chance = 100.0 - 40.0 / 4f64.powf(e - 1.0);
        if (rng.gen_range(1..=100) as f64) < bad_chance {
            println!("BAD ILLNESS---MEDICINE USED");
            state.mileage -= 5.0;
            state.misc_supplies -= 2.0;
        } else {
            println!("SERIOUS ILLNESS---");
            println!("YOU MUST STOP FOR MEDICAL ATTENTION");
            state.misc_supplies -= 10.0;
            state.illness_pending = true; // Part 1's apply_doctor_bill reads this next turn
        }
    }

    if state.misc_supplies < 0.0 {
        return Err(GameOver::RanOutOfMedicalSupplies);
    }
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
