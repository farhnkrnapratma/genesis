// Copyright (C) 2026 Farhan Kurnia Pratama
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::year::Year;
use chrono::{Datelike, Utc};

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
