use std::io;
use std::process;
use dialoguer::Select;

fn main() -> std::io::Result<()> {
    loop {
        let mut celsius = String::new();
        let mut fahrenheit = String::new();
        let selection = Select::new()
        .with_prompt("Please select the convertion you want to do")
        .item("Convert Celsius into Fahrenheit")
        .item("Convert Fahrenheit into Celsius")
        .item("Exit")
        .interact()?;
        
        match selection {
            0 => {
                io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read the line");
                let celsius: f32 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please only type numbers");
                        continue;
                    }
                };
                let fahrenheit: f32;
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
            2 => {
                process::exit(1);
            }
            _ => {}
        }
    }
}