#[allow(dead_code)]
#[path = "../src/main.rs"]
mod main_app;

use main_app::{calc_age, Year};

#[test]
fn test_year_parse_valid_ad() {
    assert_eq!(Year::parse("2026").unwrap(), Year(2026));
    assert_eq!(Year::parse("  1999  ").unwrap(), Year(1999));
    assert_eq!(Year::parse("44").unwrap(), Year(44));
}

#[test]
fn test_year_parse_valid_bc() {
    assert_eq!(Year::parse("44bc").unwrap(), Year(-44));
    assert_eq!(Year::parse("44BC").unwrap(), Year(-44));
    assert_eq!(Year::parse("-44").unwrap(), Year(-44));
    assert_eq!(Year::parse("1_200_000bc").unwrap(), Year(-1200000));
}

#[test]
fn test_year_parse_separators() {
    assert_eq!(Year::parse("1.200.000").unwrap(), Year(1200000));
    assert_eq!(Year::parse("-1.200.000").unwrap(), Year(-1200000));
    assert_eq!(Year::parse("1_200_000").unwrap(), Year(1200000));
}

#[test]
fn test_year_parse_errors() {
    // Empty
    assert_eq!(Year::parse("").unwrap_err(), "Empty input");
    assert_eq!(Year::parse("   ").unwrap_err(), "Empty input");

    // Too long
    assert_eq!(Year::parse("12345678901234567890123456").unwrap_err(), "Input too long");

    // Mixed separators
    assert_eq!(Year::parse("1.200_000").unwrap_err(), "Mixed separators");

    // Invalid combination
    assert_eq!(Year::parse("-44bc").unwrap_err(), "Invalid combination");
    assert_eq!(Year::parse("-44BC").unwrap_err(), "Invalid combination");

    // Zero BC and Year zero
    assert_eq!(Year::parse("0bc").unwrap_err(), "Zero BC");
    assert_eq!(Year::parse("0").unwrap_err(), "Year zero");
    assert_eq!(Year::parse("-0").unwrap_err(), "Year zero");

    // Invalid grouping
    assert_eq!(Year::parse("12.34").unwrap_err(), "Invalid grouping");
    assert_eq!(Year::parse("1234.567").unwrap_err(), "Invalid grouping");
    assert_eq!(Year::parse("1.23").unwrap_err(), "Invalid grouping");

    // Parse errors (non-numeric)
    assert_eq!(Year::parse("abc").unwrap_err(), "Parse error");
    assert_eq!(Year::parse("2026a").unwrap_err(), "Parse error");

    // Too old (max BC is 13_787_000_000)
    assert_eq!(Year::parse("-13.787.000.001").unwrap_err(), "Too old");
    assert_eq!(Year::parse("13.787.000.001bc").unwrap_err(), "Too old");
}

#[test]
fn test_calc_age() {
    let curr = 2026;
    
    // Normal AD
    assert_eq!(calc_age(Year(2000), curr).unwrap(), 26);
    
    // Normal BC (Year -44 to 2026 -> 2026 - (-44) - 1 = 2069)
    assert_eq!(calc_age(Year(-44), curr).unwrap(), 2069);
    
    // Born in current year
    assert_eq!(calc_age(Year(2026), curr).unwrap(), 0);
    
    // 1 BC to 1 AD
    assert_eq!(calc_age(Year(-1), 1).unwrap(), 1);
    
    // Future year error
    assert_eq!(calc_age(Year(2027), curr).unwrap_err(), "Future year");
}

#[test]
fn test_year_display() {
    let ad = Year(2026);
    assert_eq!(format!("{}", ad), "\x1b[1m2026\x1b[0m AD");

    let bc = Year(-44);
    assert_eq!(format!("{}", bc), "\x1b[1m44\x1b[0m BC");
}
