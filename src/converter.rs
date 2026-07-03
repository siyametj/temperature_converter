// temperature_converter/src/converter.rs

pub fn convert_temperature(value: f64, from_unit: char, to_unit: char) -> f64 {
    // if from init and to unit are same
    if from_unit == to_unit {
        return value;
    }

    // From any unit to celsius
    let celsius = match from_unit {
        'c' => value,                      // Celsius to Celsius
        'f' => (value - 32.0) * 5.0 / 9.0, // Fahrenheit to Celsius
        'k' => value - 273.15,             // kelvin to Celsius
        _ => value,                        // thats use as backup
    };

    // Celsius to target unit
    match to_unit {
        'c' => celsius,                      // Celsius to Celsius
        'f' => (celsius * 9.0 / 5.0) + 32.0, // Celsius to Fahrenheit
        'k' => celsius + 273.15,             // Celsius to kelvin
        _ => celsius,                        //  use as backup
    }
}

pub fn print_result(value: f64, to_unit: char, from_unit: char, result: f64) {
    println!("{:.2}°{} = {:.2}°{}", value, from_unit, result, to_unit);
}
