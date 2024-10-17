use dialoguer::Select;
use std::io;

fn main() {
    println!("Convert temperatures!");

    let options = vec!["Celsius to Fahrenheit", "Fahrenheit to Celsius"];

    let selection = Select::new()
        .with_prompt("Which conversion do you need?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    loop {
        println!("Please input a temperature:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Invalid input");

        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if selection == 0 {
            let converted_input: f32 = convert_to_fahrenheit(input);

            println!("{}°C are {}°F", input, converted_input);
            break;
        } else {
            let converted_input: f32 = convert_to_celsius(input);

            println!("{}°F are {}°C", input, converted_input);
            break;
        }
    }
}

fn convert_to_fahrenheit(celsius: f32) -> f32 {
    let fahrenheit = (celsius * 1.8) + 32.0;

    (fahrenheit * 10.0).round() / 10.0
}

fn convert_to_celsius(fahrenheit: f32) -> f32 {
    let celsius = (fahrenheit - 32.0) / 1.8;

    (celsius * 10.0).round() / 10.0
}
