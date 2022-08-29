use std::ops::Range;

use crate::solves::year::AdventOfCodeDay;

fn get_range(data: String) -> Range<i32> {
    let range: Vec<&str> = data.split("-").collect();

    let start = range[0]
        .trim()
        .parse::<i32>()
        .expect("End of range should be int")
        .max(1e5 as i32);

    let end = range[1]
        .trim()
        .parse::<i32>()
        .expect("End of range should be int")
        .min(1e6 as i32 - 1);

    start..end
}

fn validate_password(password: &str, validate_groups: bool) -> bool {
    if password.len() != 6 {
        return false;
    }

    let password = password.to_string();
    let chars = password.chars();

    let mut prev = None;
    let mut has_adjacent_repeating = false;
    let mut continuation_count = 1;

    for current in chars {
        if prev.is_none() {
            prev = Some(current);
            continue;
        }

        let prev_char = prev.unwrap();

        if prev_char == current {
            continuation_count += 1;
            if !validate_groups {
                has_adjacent_repeating = true
            }
        } else {
            if continuation_count == 2 && validate_groups {
                has_adjacent_repeating = true
            }

            continuation_count = 1;
        }

        if prev_char > current {
            return false;
        }

        prev = Some(current);
    }

    if validate_groups {
        return has_adjacent_repeating || continuation_count == 2;
    }

    has_adjacent_repeating
}

fn part1(data: String) -> String {
    let result = get_range(data)
        .filter(|item| validate_password(&item.to_string(), false))
        .count()
        .to_string();

    result
}

fn part2(data: String) -> String {
    let result = get_range(data)
        .filter(|item| {
            if validate_password(&item.to_string(), true) {
                return true;
            }

            false
        })
        .count()
        .to_string();

    result
}

#[test]
fn should_decline_lt_6_digits() {
    assert_eq!(
        validate_password("99999", false),
        false,
        "should decline less than 6 digit numbers"
    );
}

#[test]
fn should_decline_gt_6_digits() {
    assert_eq!(
        validate_password("1000000", false),
        false,
        "should decline more than 6 digit numbers"
    );
}

#[test]
fn should_decline_no_adjacent_double() {
    assert_eq!(
        validate_password("123456", false),
        false,
        "should decline when there isn't an identical pair of adjacent digits"
    );
}

#[test]
fn should_decline_decreasing_digits() {
    assert_eq!(
        validate_password("103456", false),
        false,
        "should decline when digits decrease from left to right"
    );
}

#[test]
fn should_accept_all_doubles() {
    assert_eq!(
        validate_password("112233", true),
        true,
        "should accept all doubles"
    );
}

#[test]
fn should_decline_larger_group() {
    assert_eq!(
        validate_password("123444", true),
        false,
        "should decline larger group"
    );
}

#[test]
fn should_accept_combined_doubles() {
    assert_eq!(
        validate_password("111122", true),
        true,
        "should accept combined doubles"
    );
}

#[test]
fn should_accept_valid_passwords() {
    assert_eq!(
        validate_password("122345", false),
        true,
        "should accept valid password"
    );

    assert_eq!(
        validate_password("111123", false),
        true,
        "should accept valid password"
    );

    assert_eq!(
        validate_password("111111", false),
        true,
        "should accept valid password"
    );
}

pub const SOLUTION: AdventOfCodeDay = AdventOfCodeDay {
    name: "Secure Container",
    part1: Some(part1),
    part2: Some(part2),
};
