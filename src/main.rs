// Copyright (C) 2026 Farhan Kurnia Pratama
// SPDX-License-Identifier: GPL-3.0-or-later

mod age;
mod year;

use crate::age::{calc_age, current_year};
use crate::year::{Year, format_with_dots};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        println!("Genesis Year Calculator");
        println!("Usage: enter a year to calculate time elapsed.");
        println!("Formats: 2000, 44bc, -1.200.000, 1_200_000bc");
        println!("Separators ('.' or '_') must group digits in threes.");
        return;
    }

    let curr_year = current_year();

    print!("Enter birth year: ");
    let _ = io::stdout().flush();

    let mut buf = String::new();
    if let Err(e) = io::stdin().read_line(&mut buf) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }

    let trimmed = buf.trim();
    if trimmed.eq_ignore_ascii_case("genesis") {
        println!("\x1b[1mThe Genesis\x1b[0m");
        println!(
            "Since the primal spark ignited the void, \x1b[1m13.787 billion years\x1b[0m have woven the tapestry of time."
        );
        return;
    }

    let year = match Year::parse(trimmed) {
        Ok(y) => y,
        Err(_) => {
            eprintln!("Invalid input. Please check --help or -h for valid formats.");
            std::process::exit(2);
        }
    };

    match calc_age(year, curr_year) {
        Ok(age) => {
            println!("Genesis: {} ({} years ago).", year, format_with_dots(age));
        }
        Err(_) => {
            eprintln!("Calculation error. Please check --help or -h.");
            std::process::exit(2);
        }
    }
}
