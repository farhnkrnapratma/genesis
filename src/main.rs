// Copyright (C) 2026 Farhan Kurnia Pratama
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::{Datelike, Utc};
use clap::Parser;
use std::fmt;
use std::io::{self, Write};

struct Bold<T>(T);

impl<T: fmt::Display> fmt::Display for Bold<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[1m{}\x1b[0m", self.0)
    }
}

const MAX_BC: i64 = 13_787_000_000;
const MAX_INPUT_LEN: usize = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Year(pub(crate) i64);

impl Year {
    pub(crate) fn parse(raw: &str) -> Result<Self, String> {
        let trimmed = raw.trim();
        if trimmed.len() > MAX_INPUT_LEN {
            return Err("Input too long".into());
        }

        let mut s = trimmed.to_ascii_lowercase();
        let mut is_bc = false;
        if s.ends_with("bc") {
            is_bc = true;
            s = s[..s.len() - 2].trim().to_string();
        }

        let mut is_negative = false;
        if s.starts_with('-') {
            is_negative = true;
            s = s[1..].trim().to_string();
        }

        if is_bc && is_negative {
            return Err("Invalid combination".into());
        }

        if s.is_empty() {
            return Err("Empty input".into());
        }

        let has_dots = s.contains('.');
        let has_underscores = s.contains('_');

        if has_dots && has_underscores {
            return Err("Mixed separators".into());
        }

        let clean_s = if has_dots || has_underscores {
            let sep = if has_dots { '.' } else { '_' };
            let parts: Vec<&str> = s.split(sep).collect();
            if parts[0].is_empty() || parts[0].len() > 3 {
                return Err("Invalid grouping".into());
            }
            for part in &parts[1..] {
                if part.len() != 3 {
                    return Err("Invalid grouping".into());
                }
            }
            s.replace(sep, "")
        } else {
            s
        };

        let n: i64 = clean_s.parse().map_err(|_| "Parse error")?;

        if is_bc && n == 0 {
            return Err("Zero BC".into());
        }

        let value = if is_bc || is_negative { -n } else { n };

        if value == 0 {
            return Err("Year zero".into());
        }

        if value < -MAX_BC {
            return Err("Too old".into());
        }

        Ok(Year(value))
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abs_val = self.0.abs() as u64;
        if self.0 < 0 {
            write!(f, "{} BC", Bold(abs_val))
        } else {
            write!(f, "{} AD", Bold(abs_val))
        }
    }
}

pub(crate) fn current_year() -> i64 {
    Utc::now().year() as i64
}

pub(crate) fn calc_age(birth: Year, curr_year: i64) -> Result<u64, String> {
    let y = birth.0;
    if y > curr_year {
        return Err("Future year".into());
    }
    let age: u64 = if y < 0 {
        (curr_year - y - 1) as u64
    } else {
        (curr_year - y) as u64
    };
    Ok(age)
}

#[derive(Parser)]
#[command(
    name = "genesis",
    about = "The Genesis",
    long_about = "Enter a year or 'genesis' to calculate time elapsed\n\
                  since the primal spark ignited the void (The Big Bang).\n\n\
                  EXAMPLES\n\t1999, 44bc, -1.200.000, 1_200_000bc",
    disable_version_flag = true
)]
struct Cli {
    /// Show version
    #[arg(short = 'v', long = "version")]
    version: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        println!("{} v{}", Bold("The Genesis"), env!("CARGO_PKG_VERSION"));
        return;
    }

    let curr_year = current_year();

    print!("Enter a value: ");
    let _ = io::stdout().flush();

    let mut buf = String::new();
    if let Err(e) = io::stdin().read_line(&mut buf) {
        eprintln!("error: {}", Bold(e));
        std::process::exit(1);
    }

    let trimmed = buf.trim();

    if trimmed.eq_ignore_ascii_case("genesis") {
        println!(
            "{}: Since the primal spark ignited the void — {} have woven the tapestry of time.",
            Bold("The Genesis"),
            Bold("13.7 billion years")
        );
        return;
    }

    let year = match Year::parse(trimmed) {
        Ok(y) => y,
        Err(e) => {
            eprintln!("error: {}", Bold(e));
            std::process::exit(2);
        }
    };

    match calc_age(year, curr_year) {
        Ok(age) => {
            println!("Genesis: {} ({} years ago)", year, Bold(age));
        }
        Err(e) => {
            eprintln!("error: {}", Bold(e));
            std::process::exit(2);
        }
    }

    // Sentry
    dotenvy::dotenv().ok();
    let dsn = std::env::var("SENTRY_DSN").expect("SENTRY_DSN not set");
    let _guard = sentry::init((
        dsn.as_str(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            send_default_pii: true,
            ..Default::default()
        },
    ));
}
