//! Part 3: Travel Decisions.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.

use crate::types::{EatingLevel, GameOver, GameState};
use crate::io::ask_number;
use crate::rng;
use crate::fort_hunting::shooting_challenge;


fn flipped(h: Hostility) -> Hostility {
    match h {
        Hostility::Hostile => Hostility::Friendly,
        Hostility::Friendly => Hostility::Hostile,
    }
}

fn ask_tactic() -> i32 {
    loop {
        println!("TACTICS");
        println!("(1) RUN (2) ATTACK (3) CONTINUE (4) CIRCLE WAGONS");
        let n = read_number();
        if (1..=4).contains(&n) {
            return n;
        }
    }
}

/// Three-way result shared by hostile ATTACK and hostile CIRCLE WAGONS.
fn shooting_outcome(state: &mut GameState, score: f64) {
    if score <= 1.0 {
        println!("NICE SHOOTING---YOU DROVE THEM OFF");
    } else if score > 4.0 {
        println!("LOUSY SHOT---YOU GOT KNIFED");
        state.injury_pending = true;
        println!("YOU HAVE TO SEE OL' DOC BLANCHARD");
    } else {
        println!("KINDA SLOW WITH YOUR COLT .45");
    }
}


fn rand() -> f64 {
    rng::rnd() as f64
}

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
    let cost = food_cost(level);
    if state.food - cost < 0.0 {
        return Err(()); // nothing was deducted, so nothing to refund
    }
    state.food -= cost;
    Ok(())
}
/// Adds this turn's travel distance to `mileage`.
///
/// Skeleton does nothing.
pub fn calculate_daily_mileage(state: &mut GameState) {
    let oxen_adjustment = (state.oxen_spend - 220.0) / 3.0;
    let random_bonus = rand() * 10.0;
    state.mileage += 200.0 + oxen_adjustment + random_bonus;
}
/// Rolls whether riders appear this turn. `None` means no encounter; otherwise returns
/// how the riders look.
///
/// Skeleton returns `None` (never an encounter).
/// Highest (5.0) at mile 400, tapering off in either direction.
fn encounter_threshold(mileage: f64) -> f64 {
    let x = (mileage / 100.0 - 4.0).powi(2);
    (x + 72.0) / (x + 12.0) - 1.0
}

pub fn rider_encounter_roll(mileage: f64) -> Option<Hostility> {
    if rand() * 10.0 > encounter_threshold(mileage) {
        return None;
    }

    print!("RIDERS AHEAD. THEY ");
    let looks = if rand() < 0.8 {
        Hostility::Hostile
    } else {
        print!("DON'T ");
        Hostility::Friendly
    };
    println!("LOOK HOSTILE");
    Some(looks)
}
/// Plays out the rider encounter: tactic menu, outcome, losses. Returns
/// `Err(GameOver::RiderMassacre)` if the bullets run out against hostile riders.
///
/// Skeleton does nothing and returns `Ok(())`}
   pub fn resolve_rider_encounter(state: &mut GameState, looks: Hostility) -> Result<(), GameOver> {
    // 20% chance the announced hostility is silently flipped
    let actual = if rand() < 0.2 { flipped(looks) } else { looks };

    let tactic = ask_tactic();

    match (actual, tactic) {
        // ---- hostile ----
        (Hostility::Hostile, 1) => {
            state.mileage += 20.0;
            state.misc_supplies -= 15.0;
            state.bullets -= 150.0;
            state.oxen_spend -= 40.0;
        }
        (Hostility::Hostile, 2) => {
            let score = shooting_challenge(state.skill_level);
            state.bullets -= 80.0 + 40.0 * score;
            shooting_outcome(state, score);
        }
        (Hostility::Hostile, 3) => {
            if rand() < 0.2 {
                println!("THEY DID NOT ATTACK");
                return Ok(());
            }
            state.bullets -= 150.0;
            state.misc_supplies -= 15.0;
        }
        (Hostility::Hostile, _) => {
            let score = shooting_challenge(state.skill_level);
            state.bullets -= 80.0 + 30.0 * score;
            state.mileage -= 25.0;
            shooting_outcome(state, score);
        }

        // ---- friendly ----
        (Hostility::Friendly, 1) => {
            state.oxen_spend -= 10.0;
            state.mileage += 15.0;
        }
        (Hostility::Friendly, 2) => {
            state.bullets -= 100.0;
            state.mileage -= 5.0;
        }
        (Hostility::Friendly, 3) => {}
        (Hostility::Friendly, _) => {
            state.mileage -= 20.0;
        }
    }
    // closing line uses the RESOLVED hostility
    match actual {
        Hostility::Hostile => println!("RIDERS WERE HOSTILE--CHECK FOR LOSSES"),
        Hostility::Friendly => println!("RIDERS WERE FRIENDLY, BUT CHECK FOR POSSIBLE LOSSES"),
    }

    if actual == Hostility::Hostile && state.bullets < 0.0 {
        println!("YOU RAN OUT OF BULLETS AND GOT MASSACRED BY THE RIDERS");
        return Err(GameOver::RiderMassacre);
    }
    Ok(())
}
