use std::io; // We need it to convert string into f32
use std::process; // We need it to exit the project
use dialoguer::Select; // We need it to have a usefull select in term

fn main() -> std::io::Result<()> {
    // I recommend to add all your code into the loop if you don't want to exit after the first run
    loop {
        // We declare these as strings because of the io::stdin
        let mut celsius = String::new();
        let mut fahrenheit = String::new();
        // We open the select term and give the result of answer to "selection" variable
        let selection = Select::new()
        .with_prompt("Please select the convertion you want to do")
        .item("Convert Celsius into Fahrenheit")
        .item("Convert Fahrenheit into Celsius")
        .item("Exit")
        .interact()?;
        
        // We can use "if" but "match" works faster.
        // You match the index of "selection"
        match selection {
            0 => {
                // Listner, it permit to the user to type the number he want to convert
                // We give the result to "celsius" variable
                io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read the line");
                // Result of "celsius" is converted into float by shadowing it
                let celsius: f32 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please only type numbers");
                        continue;
                    }
                };
                // We also shadow "fahenrheit" to make it float
                let fahrenheit: f32;
                // Do I need to epxplain this line ?
                let celsius_to_fahrenheit_formule = (celsius * 9.0/5.0) + 32.0;
                fahrenheit = celsius_to_fahrenheit_formule;
                println!("{celsius}°C = {fahrenheit}°F");
            }
            1 => {
                io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read the line");
                let fahrenheit: f32 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please only type numbers");
                        continue;
                    }
                };
                let celsius: f32;
                let fahrenheit_to_celsius_formule = (fahrenheit - 32.0) * 5.0/9.0;
                celsius = fahrenheit_to_celsius_formule;
                println!("{fahrenheit}°F = {celsius}°C");
            }
            // If the user have selected "exit", exit from the app
            2 => {
                process::exit(1);
            }
            _ => {}
        }
    }
}