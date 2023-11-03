use std::io;
fn main() {
    println!("Do you want to convert: ");
    println!("1. Farenheit to Celsius");
    println!("2. Celsius to Farenheit");
    println!("3. Exit");
    loop {
        let mut choice: String = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid choice.");
                continue;
            }
        };
        match choice {
            1 => {
                let mut farenheit: String = String::new();
                println!("Enter the temperature in Farenheit: ");
                io::stdin()
                    .read_line(&mut farenheit)
                    .expect("Failed to read line");
                let farenheit: f64 = farenheit.trim().parse().expect("Please type a number!");
                println!(
                    "{} Farenheit is {} Celsius",
                    farenheit,
                    farenheit_to_celsius(farenheit as f64)
                );
            }
            2 => {
                let mut celsius: String = String::new();
                println!("Enter the temperature in Celsius: ");
                io::stdin()
                    .read_line(&mut celsius)
                    .expect("Failed to read line");
                let celsius: f64 = celsius.trim().parse().expect("Please type a number!");
                println!(
                    "{} Celsius is {} Farenheit",
                    celsius,
                    celcius_to_farenheit(celsius as f64)
                );
            }
            3 => break,
            _ => {
                println!("Please enter a valid choice");
                continue;
            }
        }
    }
}

fn farenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celcius_to_farenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
