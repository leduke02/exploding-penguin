//! Part 4: Events, Mountains & Endings.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.
//!
//! On a 3-person team, nobody owns this part. Leave this file as it is: it is the
//! placeholder module, it is ungraded, and the rest of the game builds and runs against it.

use crate::types::{EatingLevel, EventKind, GameOver, GameState};

/// Rolls a random percentage and returns the matching event from the event table.
pub fn roll_random_event() -> EventKind {
    let roll: i32 = rand::random_range(0..100);
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

/// Random whole number in 0..=n, as an f64 ("lose 30 plus up to 40 random miles").
fn up_to(n: i32) -> f64 {
    rand::random_range(0..=n) as f64
}

/// A blizzard in the pass: sets the flag, costs supplies and miles, and may cause
/// illness from exposure.
fn blizzard(state: &mut GameState) -> Result<(), GameOver> {
    println!("BLIZZARD IN MOUNTAIN PASS--TIME AND SUPPLIES LOST");
    state.blizzard_this_turn = true;
    state.food -= 25.0;
    state.misc_supplies -= 10.0;
    state.bullets -= 300.0;
    state.mileage -= 30.0 + up_to(40);

    // Clothing below a small, slightly randomized threshold (about 18-20) means exposure.
    let threshold = 18.0 + 2.0 * rand::random_range(0.0..1.0);
    if state.clothing < threshold {
        illness_check(state)?;
    }
    Ok(())
}

/// Mountain crossing past mile 950: rugged mountains, South Pass, Blue Mountains
pub fn cross_mountains(state: &mut GameState) -> Result<(), GameOver> {
   if state.mileage <= 950.0 {
        return Ok(());
    }


    // Rugged mountains: most likely near mile 1500, less likely farther away.
    let d = state.mileage / 100.0 - 15.0;
    let threshold = 9.0 - (d * d + 72.0) / (d * d + 12.0);
    if rand::random_range(0.0..10.0) > threshold {
        println!("RUGGED MOUNTAINS");
        if rand::random_range(0.0..1.0) < 0.10 {
            println!("YOU GOT LOST---LOSE VALUABLE TIME TRYING TO FIND TRAIL!");
            state.mileage -= 60.0;
        } else if rand::random_range(0.0..1.0) < 0.11 {
            println!("WAGON DAMAGED!---LOSE TIME AND SUPPLIES");
            state.misc_supplies -= 5.0;
            state.bullets -= 200.0;
            state.mileage -= 20.0 + up_to(30);
        } else {
            println!("THE GOING GETS SLOW");
            state.mileage -= 45.0 + up_to(50);
        }
    }

    // South Pass: one-time checkpoint, 80% blizzard chance.
    if !state.south_pass_cleared {
        state.south_pass_cleared = true;
        if rand::random_range(0.0..1.0) < 0.80 {
            blizzard(state)?;
        } else {
            println!("YOU MADE IT SAFELY THROUGH SOUTH PASS--NO SNOW");
        }
    }

    // Blue Mountains: one-time checkpoint once mileage reaches 1700, 70% blizzard chance.
    if state.mileage >= 1700.0 && !state.blue_mountains_cleared {
        state.blue_mountains_cleared = true;
        if rand::random_range(0.0..1.0) < 0.70 {
            blizzard(state)?;
        } else {
            // Matches the flowchart: same safe message as South Pass.
            println!("YOU MADE IT SAFELY THROUGH SOUTH PASS--NO SNOW");
        }
    }

    Ok(())
}

/// Poor = 1, Moderate = 2, Well = 3 (the "EatingLevel" in the formulas).
fn eating_number(level: EatingLevel) -> f64 {
    match level {
        EatingLevel::Poor => 1.0,
        EatingLevel::Moderate => 2.0,
        EatingLevel::Well => 3.0,
      }
}

/// Rolls illness severity. Returns `Err(GameOver::RanOutOfMedicalSupplies)` when the
/// medicine runs out.
pub fn illness_check(state: &mut GameState) -> Result<(), GameOver> {
    let e = eating_number(state.last_eating_level);

    // Roll 1: mild?  1..100 < 10 + 35 * (e - 1)
    let mild_chance = 10.0 + 35.0 * (e - 1.0);
    if (rand::random_range(1..=100) as f64) < mild_chance {
        println!("MILD ILLNESS---MEDICINE USED");
        state.mileage -= 5.0;
        state.misc_supplies -= 2.0;
    } else {
        // Roll 2: bad vs serious.  1..100 < 100 - 40 / 4^(e - 1)
        let bad_chance = 100.0 - 40.0 / 4f64.powf(e - 1.0);
        if (rand::random_range(1..=100) as f64) < bad_chance {
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
