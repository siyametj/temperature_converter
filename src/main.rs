// temperature_converter/src/main.rs

mod converter;
mod helper;
mod input_validator;

fn main() {
    helper::clear_screen();
    println!("=====================================");
    println!("WELCOME TO TEMPERATURE CONVERTER");
    println!("=====================================");

    loop {
        println!("\nStart new conversion...");

        let value = input_validator::get_valid_float("Enter temperature value:");

        let from_unit = input_validator::get_valid_unit("Enter current unit (C, F, K):");

        let to_unit = input_validator::get_valid_unit("Enter target unit (C, F, K):");

        let result = converter::convert_temperature(value, from_unit, to_unit);

        converter::print_result(value, from_unit, result, to_unit);

        let run_again =
            input_validator::get_valid_choice("Do you want to calculate again? (yes/no):");

        if run_again {
            helper::clear_screen();
            println!("TEMPERATURE CONVERTER (RESTARTED)");
        } else {
            println!("Thanks for using this program. Goodbye!");
        }
    }
}
