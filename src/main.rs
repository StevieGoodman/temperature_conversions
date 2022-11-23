use std::io;

fn main() {
    loop {
        let temp_in_fahrenheit = get_f32_input("Please select a temperature (in fahrenheit) to convert to celsius: ");
        let temp_in_celsius = fahrenheit_to_celsius(temp_in_fahrenheit);
        println!("{}ºF is {}ºC", temp_in_fahrenheit, temp_in_celsius);

        let temp_in_celsius = get_f32_input("Please select a temperature (in celsius) to convert to fahrenheit: ");
        let temp_in_fahrenheit = celsius_to_fahrenheit(temp_in_celsius);
        println!("{}ºC is {}ºF", temp_in_celsius, temp_in_fahrenheit);
    }
}

/// Requests input from the user and returns valid input as an f32 value. Loops if invalid input is provided.
fn get_f32_input(input_message: &str) -> f32 {
    loop {
        println!("{}", input_message);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(msg) => {
                eprintln!("{}", msg);
                continue;
            }
        }
        let input: f32 = match input.trim().parse() {
            Ok(value) => value,
            Err(msg) => {
                eprintln!("{}", msg);
                continue;
            }
        };
        break input
    }
}

/// Converts a temperature represented in fahrenheit and returns the same temperature represented in celsius.
fn fahrenheit_to_celsius(temp_in_fahrenheit: f32) -> f32 {
    let temp_in_celsius = (temp_in_fahrenheit - 32f32) * (5f32/9f32);
    temp_in_celsius
}

/// Converts a temperature represented in celsius and returns the same temperature represented in fahrenheit.
fn celsius_to_fahrenheit(temp_in_celsius: f32) -> f32 {
    let temp_in_fahrenheit = (temp_in_celsius /  (5f32/9f32)) + 32f32;
    temp_in_fahrenheit
}
