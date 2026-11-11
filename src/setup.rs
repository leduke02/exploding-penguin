//! Part 1: Foundation & Setup.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.
use crate::io;
use crate::types::{GameOver, GameState};

/// The 19 turn dates, two weeks apart. Turn 1 is DATES[0].
const DATES: [&str; 19] = [
    "APRIL 12",
    "APRIL 26",
    "MAY 10",
    "MAY 24",
    "JUNE 7",
    "JUNE 21",
    "JULY 5",
    "JULY 19",
    "AUGUST 2",
    "AUGUST 16",
    "AUGUST 30",
    "SEPTEMBER 13",
    "SEPTEMBER 27",
    "OCTOBER 11",
    "OCTOBER 25",
    "NOVEMBER 8",
    "NOVEMBER 22",
    "DECEMBER 6",
    "DECEMBER 20",
];
/// Prints the skill menu, reads the choice with `io::ask_number`, and returns it.
/// A value above 5 silently becomes 0 (see the Part 1 page).
///
/// Skeleton returns 3.0 ("fair to middlin'").
pub fn get_shooting_skill_level() -> f64 {
    println!("HOW GOOD A SHOT ARE YOU WITH YOUR RIFLE?");
    println!("  (1) ACE MARKSMAN,  (2) GOOD SHOT,  (3) FAIR TO MIDDLIN'");
    println!("  (4) NEED MORE PRACTICE,  (5) SHAKY KNEES");
    println!("ENTER ONE OF THE ABOVE -- THE BETTER YOU CLAIM YOU ARE, THE");
    println!("FASTER YOU'LL HAVE TO BE WITH YOUR GUN TO BE SUCCESSFUL.");
 
    let skill = io::ask_number();
 
    // Silent: no error message, no re-prompt. 5 or below (including 0 and
    // negatives) passes through unchanged.
    if skill > 5.0 {
        0.0
    } else {
        skill
    }
}

/// Runs the whole initial-purchase sequence and returns the starting `GameState`.
///
/// Skeleton returns a generic, plausible starting state so other parts can be tested.
pub fn purchase_supplies(skill_level: f64) -> GameState {
    let (oxen, food, ammo, clothing, misc) = 'buying: loop {
        // --- Oxen: must be between $200 and $300 ---
        let oxen = loop {
            println!("HOW MUCH DO YOU WANT TO SPEND ON YOUR OXEN TEAM");
            let v = io::ask_number();
            if v < 200.0 {
                println!("NOT ENOUGH");
            } else if v > 300.0 {
                println!("TOO MUCH");
            } else {
                break v;
            }
        };
 
        // --- Food ---
        let food = loop {
            println!("HOW MUCH DO YOU WANT TO SPEND ON FOOD");
            let v = io::ask_number();
            if v < 0.0 {
                println!("IMPOSSIBLE");
            } else {
                break v;
            }
        };
 
        // --- Ammunition ---
        let ammo = loop {
            println!("HOW MUCH DO YOU WANT TO SPEND ON AMMUNITION");
            let v = io::ask_number();
            if v < 0.0 {
                println!("IMPOSSIBLE");
            } else {
                break v;
            }
        };
 
        // --- Clothing ---
        let clothing = loop {
            println!("HOW MUCH DO YOU WANT TO SPEND ON CLOTHING");
            let v = io::ask_number();
            if v < 0.0 {
                println!("IMPOSSIBLE");
            } else {
                break v;
            }
        };
 
        // --- Miscellaneous supplies ---
        let misc = loop {
            println!("HOW MUCH DO YOU WANT TO SPEND ON MISCELLANEOUS SUPPLIES");
            let v = io::ask_number();
            if v < 0.0 {
                println!("IMPOSSIBLE");
            } else {
                break v;
            }
        };
 
        // --- Overspent? Restart all five questions from the oxen prompt ---
        if oxen + food + ammo + clothing + misc > 700.0 {
            println!("YOU OVERSPENT--YOU ONLY HAD $700 TO SPEND. BUY AGAIN");
            continue 'buying;
        }
 
        break (oxen, food, ammo, clothing, misc);
    };
 
    let bullets = ammo * 50.0;
    let cash = 700.0 - (oxen + food + ammo + clothing + misc);
 
    println!("AFTER ALL YOUR PURCHASES, YOU NOW HAVE");
    println!("{} DOLLARS LEFT", cash);
    println!("MONDAY MARCH 29 1847");
 // mileage = 0, turn_number = 0, and all flags (illness_pending,
    // injury_pending, south_pass_cleared, blue_mountains_cleared,
    // fort_available_next_turn) start cleared via Default.
    GameState {
        food,
        bullets,
        clothing,
        misc_supplies: misc,
        cash,
        oxen_spend: oxen,
        skill_level,
        ..GameState::default()
    }
} 

/// Advances to the next turn and prints its date. Returns `Err(GameOver::TrailTooLong)`
/// when the trip runs past the last date in the table.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn advance_turn_date(state: &mut GameState) -> Result<(), GameOver> {
    state.turn_number += 1;
 
    if state.turn_number >= 20 {
        println!("YOU HAVE BEEN ON THE TRAIL TOO LONG ------");
        println!("YOUR FAMILY DIES IN THE FIRST BLIZZARD OF WINTER");
        return Err(GameOver::TrailTooLong); // caller must end the game
    }
 
    let date = DATES[(state.turn_number - 1) as usize];
    println!();
    println!("MONDAY {} 1847", date);
    println!();
    Ok(())
}

/// Start-of-turn cleanup and the status table: clamp negatives, low-food warning, floor
/// values, record `turn_start_mileage`, print mileage and the resource table.
///
/// Skeleton does nothing.
pub fn print_status_table(state: &mut GameState) {
    // Clamp negatives up to 0.
    state.food = state.food.max(0.0);
    state.bullets = state.bullets.max(0.0);
    state.clothing = state.clothing.max(0.0);
    state.misc_supplies = state.misc_supplies.max(0.0);
 
    if state.food < 13.0 {
        println!("YOU'D BETTER DO SOME HUNTING OR BUY FOOD SOON!!!!");
    }
 
    // Floor to whole numbers.
    state.food = state.food.floor();
    state.bullets = state.bullets.floor();
    state.clothing = state.clothing.floor();
    state.misc_supplies = state.misc_supplies.floor();
    state.cash = state.cash.floor();
    state.mileage = state.mileage.floor();
    state.turn_start_mileage = state.mileage;
 
    if state.mountain_crossed {
        println!("TOTAL MILEAGE IS 950");
        state.mountain_crossed = false;
    } else {
        println!("TOTAL MILEAGE IS {}", state.mileage);
    }
 
    println!("FOOD\tBULLETS\tCLOTHING\tMISC. SUPP.\tCASH");
    println!(
        "{}\t{}\t{}\t\t{}\t\t{}",
        state.food, state.bullets, state.clothing, state.misc_supplies, state.cash
    );
}

/// Charges the $20 doctor's bill when an illness or injury flag is set. Returns
/// `Err(GameOver::CantAffordDoctor)` when cash cannot cover it.
///
/// Skeleton does nothing and returns `Ok(())`.
pub fn apply_doctor_bill(state: &mut GameState) -> Result<(), GameOver> {
    if !(state.illness_pending || state.injury_pending) {
        return Ok(());
    }
 
    state.cash -= 20.0;
    println!("DOCTOR'S BILL IS $20");
    state.illness_pending = false;
    state.injury_pending = false;
 
    if state.cash < 0.0 {
        return Err(GameOver::CantAffordDoctor); // Part 4 prints YOU CAN'T AFFORD A DOCTOR
    }
    Ok(())
}
