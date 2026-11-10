//! Console input/output helpers.
//!
//! Classic BASIC `INPUT` prints a `?` prompt and re-asks on bad numeric input. These
//! helpers reproduce that behaviour so the rest of the program never touches `stdin`.

use std::io::{self, BufRead, Write};

/// Reads one line from standard input. Exits quietly if input has ended (EOF).
pub fn read_line() -> String {
    io::stdout().flush().ok();
    let mut line = String::new();
    match io::stdin().lock().read_line(&mut line) {
        Ok(0) | Err(_) => {
            println!();
            std::process::exit(0);
        }
        Ok(_) => line.trim().to_string(),
    }
}

/// BASIC `PRINT "prompt"; : INPUT x$` — prints the prompt followed by `? ` and reads text.
pub fn ask(prompt: &str) -> String {
    print!("{}? ", prompt);
    read_line()
}

/// Like [`ask`], but the answer must be a number. Re-asks until it is.
pub fn ask_number(prompt: &str) -> f64 {
    loop {
        let text = ask(prompt);
        match text.parse::<f64>() {
            Ok(n) if n.is_finite() => return n,
            _ => println!("?REDO FROM START"),
        }
    }
}

/// Formats a number the way BASIC would: whole numbers without a decimal point.
pub fn fmt_num(n: f64) -> String {
    if n.fract() == 0.0 {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}

/// BASIC `TAB(n)`: moves the print position to column `n`.
pub fn tab(n: usize) -> String {
    " ".repeat(n)
}
