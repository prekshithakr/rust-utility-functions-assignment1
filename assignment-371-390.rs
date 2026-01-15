// Assignment: Utility Functions (371–390)
// Language: Rust
// Purpose: Practice basic Rust programming
// Author: Prekshitha K R
// Note: Code written for learning purposes with simple logic

371. Generate Random ID
use rand::Rng;

fn random_id(length: usize) -> String {
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let mut id = String::new();

    let mut i = 0;
    while i < length {
        let index = rng.gen_range(0..characters.len());
        let ch = characters.chars().nth(index).unwrap();
        id.push(ch);
        i += 1;
    }

    id
}

372. Generate Random String (custom characters)
fn random_string(length: usize, charset: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut result = String::new();

    for _ in 0..length {
        let index = rng.gen_range(0..charset.len());
        result.push(charset.chars().nth(index).unwrap());
    }

    result
}

373. Generate Random Boolean
fn random_boolean() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.5)
}

374. Generate Random Integer
fn random_integer(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

375. Generate Random Float (with precision)
fn random_float(min: f64, max: f64, precision: u32) -> f64 {
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(min..max);

    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

376. Generate Random Item from Array
fn random_item(arr: &[i32]) -> i32 {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..arr.len());
    arr[index]
}

377. Generate Random Date
use chrono::{NaiveDate, Duration};

fn random_date(start: NaiveDate, end: NaiveDate) -> NaiveDate {
    let days_between = (end - start).num_days();
    let mut rng = rand::thread_rng();
    let random_days = rng.gen_range(0..=days_between);

    start + Duration::days(random_days)
}

378. Generate Random Color
fn random_color() -> String {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0..=0xFFFFFF);
    format!("#{:06X}", number)
}

379. Format Bytes
fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{} Bytes", bytes);
    }

    let kb = bytes as f64 / 1024.0;
    format!("{:.2} KB", kb)
}

380. Format Number
fn format_number(num: f64) -> String {
    format!("{:,.2}", num)
}

381. Format Currency
fn format_currency(amount: f64, currency: &str) -> String {
    format!("{} {:.2}", currency, amount)
}

382. Format Percentage
fn format_percentage(num: f64) -> String {
    let percent = num * 100.0;
    format!("{:.2}%", percent)
}

383. Pluralize
fn pluralize(count: i32, word: &str) -> String {
    if count == 1 {
        format!("{} {}", count, word)
    } else {
        format!("{} {}s", count, word)
    }
}

384. Truncate Number (Rust)
  fn truncate_number(num: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (num * factor).trunc() / factor
}


385. Clamp Number
fn clamp(num: i32, min: i32, max: i32) -> i32 {
    if num < min {
        min
    } else if num > max {
        max
    } else {
        num
    }
}

386. Normalize Number
fn normalize(num: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    let ratio = (num - old_min) / (old_max - old_min);
    ratio * (new_max - new_min) + new_min
}

387. Round to Precision
fn round_to_precision(num: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (num * factor).round() / factor
}

388. Is Power of Two
fn is_power_of_two(n: u32) -> bool {
    if n == 0 {
        return false;
    }
    (n & (n - 1)) == 0
}

389. Next Power of Two
fn next_power_of_two(n: u32) -> u32 {
    n.next_power_of_two()
}

390. Fibonacci Sequence
fn fibonacci_sequence(n: usize) -> Vec<u64> {
    let mut result = Vec::new();

    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        result.push(a);
        let c = a + b;
        a = b;
        b = c;
    }

    result
}
