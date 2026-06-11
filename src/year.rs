// Copyright (C) 2026 Farhan Kurnia Pratama
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt;

const MAX_BC: i64 = 13_787_000_000;
const MAX_INPUT_LEN: usize = 25;

pub(crate) fn format_with_dots(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().rev().collect();
    for (i, c) in chars.into_iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push('.');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

#[derive(Debug, Clone, Copy)]
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
        let formatted = format_with_dots(abs_val);
        if self.0 < 0 {
            write!(f, "{} BC", formatted)
        } else {
            write!(f, "{} AD", formatted)
        }
    }
}
