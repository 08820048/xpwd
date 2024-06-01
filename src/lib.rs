use colored::*;
use prettytable::{cell, row, Table};
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use zxcvbn::zxcvbn;

/// Generates a random unsigned 32-bit integer within a specified range
///
/// # Arguments
///
/// * `min` - Lower bound of the range (inclusive)
/// * `max` - Upper bound of the range (inclusive)
///
/// # Returns
///
/// A randomly generated `u32` value within the specified range
fn gen_random_u32(min: u32, max: u32) -> u32 {
    thread_rng().gen_range(min..=max)
}

/// Generates a floating-point random number within a given range and decimal places
///
/// # Arguments
///
/// * `min` - Lower limit of the range
/// * `max` - Upper limit of the range
/// * `decimal_places` - Number of decimal places for the float
///
/// # Returns
///
/// Randomly generated floating-point number with the specified precision
fn gen_random_f64(min: f64, max: f64, decimal_places: u32) -> f64 {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(min..=max);
    let factor = 10f64.powi(decimal_places as i32);
    ((rand_num * factor).round() / factor)
}

// Generates a random string of a specified length
fn gen_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMERIC_CHARS: &str = "0123456789";
const SPECIAL_CHARS: &str = "!@#$%^&*()-_=+[{]};:,<.>/?";

/// Generates a password of a given length and complexity level
///
/// # Arguments
///
/// * `len` - Length of the password to generate
/// * `complexity` - Complexity level ('s' for simple, 'm' for medium, 'c' for complex)
///
/// # Returns
///
/// A password string meeting the specified complexity requirements
pub fn gen_password(len: usize, complexity: &str) -> String {
    let mut rng = thread_rng();
    let mut all_chars: Vec<char> = Vec::with_capacity(len);

    match complexity {
        "s" => {
            all_chars.extend(LOWERCASE_CHARS.chars());
            all_chars.extend(NUMERIC_CHARS.chars());
        }
        "m" => {
            all_chars.extend(LOWERCASE_CHARS.chars());
            all_chars.extend(UPPERCASE_CHARS.chars());
            all_chars.extend(NUMERIC_CHARS.chars());
        }
        "c" => {
            all_chars.extend(LOWERCASE_CHARS.chars());
            all_chars.extend(UPPERCASE_CHARS.chars());
            all_chars.extend(NUMERIC_CHARS.chars());
            all_chars.extend(SPECIAL_CHARS.chars());
        }
        _ => panic!("Invalid complexity level: {}", complexity),
    }

    (0..len)
        .map(|_| *all_chars.choose(&mut rng).unwrap())
        .collect()
}

pub fn print_data_tables(datas: &[(String, String, String)]) {
    let mut table = Table::new();
    table.add_row(row![cell!("Len"), cell!("complex"), cell!("password")]);

    for (p1, p2, p3) in datas {
        table.add_row(row![p1, p2, p3]);
    }
    table.printstd();
}

// Evaluates the strength of a given password
fn evaluate_password_strength(password: &str) -> u8 {
    let result = zxcvbn(password, &[]);
    result.score().into()
}

// Prints a visual representation of the password strength
pub fn print_password_strength(password: &str) {
    let score = evaluate_password_strength(password);
    print!("🛡️ ");
    match score {
        0 => println!("{} {}", "■■■■■■■■".red(), "very weak".red()), // Weak
        1 => println!("{} {}", "■■■■■■■■■■■■■".magenta(), "weak".magenta()), // Very Weak
        2 => println!(
            "{} {}",
            "■■■■■■■■■■■■■■■■■■■■".yellow(),
            "moderate".yellow()
        ), // Weak
        3 => println!(
            "{} {}",
            "■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■".cyan(),
            "strong".cyan()
        ), // Medium
        4 => println!(
            "{} {}",
            "■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■".green(),
            "very strong".green()
        ), // Strong
        _ => unreachable!(),                                         // Handle any unexpected cases
    }
}
