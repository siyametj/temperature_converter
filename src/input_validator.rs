// temperature_converter/src/input_validator.rs

use std::io::{self, Write};

// Function for take float64 temperature from user
pub fn get_valid_float(prompt: &str) -> f64 {
    let mut input = String::new();

    loop {
        print!("{} ", prompt);
        let _ = io::stdout().flush().expect("Failed to flush");

        input.clear();

        // if user give ctrl + d
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nProgram interrupted by user! Goodbye!");
                std::process::exit(0);
            }

            Ok(_) => {
                let trimmed = input.trim();

                // if user not input anything
                if trimmed.is_empty() {
                    println!("Input can't be empty! Please try again.");
                    continue;
                }

                match trimmed.parse::<f64>() {
                    Ok(num) => return num, // convert in float

                    // if user give wrong number(e.g., not integer)
                    Err(_) => {
                        println!(
                            "'{}' not an valid number. Please enter a valid number.",
                            trimmed
                        );
                        continue;
                    }
                }
            }

            Err(_) => {
                println!("Something went wrong in input! Please try again!");
                continue;
            }
        }
    }
}

// Function for take unit of temperature
pub fn get_valid_unit(prompt: &str) -> char {
    let mut input = String::new();
    loop {
        print!("{} ", prompt);
        let _ = io::stdout().flush().expect("Failed to flush");

        input.clear();

        match io::stdin().read_line(&mut input) {
            // EOF or ctrl + d press by user
            Ok(0) => {
                println!("\nProgram interrupted by user! Goodbye!");
                std::process::exit(0);
            }

            Ok(_) => {
                let trimmed = input.trim().to_lowercase();

                // A temperature unit length is 1 char so if user input more than 1 char
                if trimmed.len() != 1 {
                    println!(
                        "Temperature unit length can't be more than 1 character. Please try again!"
                    );
                    continue;
                }

                let unit = trimmed.chars().next().unwrap();

                if unit == 'c' || unit == 'k' || unit == 'f' {
                    return unit;
                } else {
                    println!("Invalid unit! Please try again with c or f or k");
                    continue;
                }
            }

            Err(_) => {
                println!("Something went wrong! Please try again!");
                continue;
            }
        }
    }
}

pub fn get_valid_choice(prompt: &str) -> bool {
    let mut input = String::new();

    loop {
        print!("{} ", prompt);
        let _ = io::stdout().flush().expect("Failed to flush");

        input.clear();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("Program interrupted by user! Goodbye!");
                std::process::exit(0);
            }

            Ok(_) => {
                let trimmed = input.trim().to_lowercase();

                if trimmed == "yes" || trimmed == "y" {
                    return true; // for run again
                } else if trimmed == "no" || trimmed == "n" {
                    return false; // for not run again
                } else {
                    println!("Invalid input! Please input y/yes or n/no");
                    continue;
                }
            }

            Err(_) => {
                println!("Something went wrong! Please try again!");
                continue;
            }
        }
    }
}

