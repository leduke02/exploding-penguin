//! Part 2: Fort, Hunting & Shooting.
//!
//! Every function below is a skeleton: it compiles and returns a harmless default so the
//! other parts can build against it. Replace the body with your own code. Keep the
//! signature unless your team and instructor agree to change it.

use crate::types::{GameState, TurnChoice};
use crate::io::{read_line, ask, ask_number, fmt_num,};
use crate::rng::rnd;
/// Shows the right turn menu (fort or no fort), settles a valid, affordable choice, flips
/// `fort_available_next_turn`, and returns the choice.
///
/// Skeleton returns `TurnChoice::Continue`.
pub fn turn_menu(state: &mut GameState) -> TurnChoice {
    let mut player_input: i32 = 0;
    if state.turn_number%2 == 0{
	println!("DO YOU WANT TO (1) STOP AT THE NEXT FORT, (2) HUNT, OR (3)continue");	    
        loop{	    
            if player_input < 1{
	        player_input = fmt_num(ask_number("")).parse::<i32>().expect("REASON");
            }else{break;} 	
        }    
        if player_input == 1{
	    return TurnChoice::StopAtFort;
        }else if player_input == 2{
	    return TurnChoice::Hunt;
        }else{
	    return TurnChoice::Continue;    
	} 
    }else{
    	println!("DO YOU WANT TO (1) HUNT, OR (2) CONTINUE");
        loop{	    
            if player_input < 1{
	        player_input = fmt_num(ask_number("")).parse::<i32>().expect("REASON");
            }else{break;} 	
        }    
        if player_input == 1{
	    return TurnChoice::Hunt
        }else{
	    return TurnChoice::Continue
        }	
    }
}

/// Fort purchases at the worse 2:3 exchange rate; stopping costs 45 miles.
///
/// Skeleton does nothing.
pub fn fort_purchases(state: &mut GameState) {
    let mut player_input: f64 = -10.0;
    let mut repitition_count: i32;
    let resources = ["FOOD", "AMMUNITION", "CLOTHING", "MISC"]; 
    println!("ENTER WHAT YOU WISH TO SPEND ON THE FOLLOWING");
    for i in 0..4{
	println!("{}:",resources[i]);
	loop{	    
	    if player_input < 0.0{
      	   player_input = ask_number("");
  	  }else{break;} 	
	}
	if player_input > state.cash{
	    println!("YOU DON'T HAVE THAT MUCH--KEEP YOUR SPENDING DOWN");
	    println!("YOU MISS YOUR CHANCE TO SPEND ON THAT ITEM");
	}else{
    	    state.cash -= player_input;
	    repitition_count = i as i32;
	    match repitition_count{
		0 => state.food += player_input*(2.0/3.0),
		1 => state.bullets += player_input*(2.0/3.0),
		2 => state.clothing += player_input*(2.0/3.0) ,
		3 => state.misc_supplies += player_input*(2.0/3.0),
		_ => player_input = 0.0,
	    };
	}
    }
}
/// Hunting: bullet check, 45 miles, shooting challenge, then `resolve_hunt_outcome`.
///
/// Skeleton does nothing.
pub fn go_hunting(state: &mut GameState){
    if state.bullets > 40.0{
	let score: f64 = shooting_challenge(state.skill_level);
	resolve_hunt_outcome(score,state);
	state.mileage -= 45.0;
    }else{
	println!("TOUGH---YOU NEED MORE BULLETS TO GO HUNTING");
	return
    }
}

/// The timed "type the word" challenge. Returns the response score: lower is faster, and
/// 9 means the wrong word was typed. Also called from Parts 3 and 4.
///
/// Skeleton returns 2.0 (a middling response) without prompting.
pub fn shooting_challenge(skill_level: f64) -> f64 {
    let gun_word = ["BANG", "BLAM", "POW", "WHAM"];
    let roll = (rnd() * gun_word.len() as f64).floor() as usize;
    let mut score: f64;
    println!("Type: {}", gun_word[roll]); 
    // take initial time;
    let player_input = read_line();
    if player_input == gun_word[roll]{
	//take secondary time;
	let time_taken: f64 = second_time - initial_time;  
	score = time_taken/skill_level;
    }else{
	score = 9.0;
    }
score
}

/// Turns a response score into a hunt result: food gained, bullets spent, or a miss.
///
/// Skeleton does nothing.
pub fn resolve_hunt_outcome(score: f64, state: &mut GameState) {
    if score <= 1.0{
	state.food += 52.0 + (rnd()*6.0).floor() as f64;
	state.bullets -=10.0+(rnd()*4.0).floor() as f64;
	println!("RIGHT BETWEEN THE EYES---YOU GOT A BIG ONE!!!!");
	println!("FULL BELLIES TONIGHT!");
    }else if 48.0 >= score*2.0 {
	state.food += 48.0 - score*2.0;
	state.bullets -= 10.0+score*3.0;
	println!("NICE SHOT---RIGHT ON TARGET---GOOD EATIN' TONIGHT!!");
    }else{
	state.bullets -= 10.0+score*3.0;
	println!("YOU MISSED---AND YOUR DINNER GOT AWAY.....");
    }
}
