use std::io;
use std::io::Write;

fn main() {
    loop {

        println!("\n---Please enter which unit you want to convert to and temp of the opposite \
                     unit---\n");


        print!("Type in \"C\" to convert to Celsius or \"F\" to convert to Fahrenheit: ");
        let temp_sign = input_in().trim().to_uppercase();


        print!("Please enter the temperature(must be of the opposite unit):  ");
        let temp = input_in();
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nDid not receive a numerical temperature. Restarting...");
                continue;
            }
        };

        match temp_sign.as_str() {
            "C" => {
                let results = convert_to_celsius(temp);
                println!("{}°Fahrenheit is {:.4}°Celsius\n", temp, results);
            }

            "F" => {
                let results = convert_to_fahrenheit(temp);
                println!("{}°Celsius is {:.4}°Fahrenheit\n", temp, results);
            }

            _ => {
                println!("\nDid not receive proper unit \"C\" or \"F\". Restarting...\n");
                continue;
            }
        }

        print!("To continue converting for more temps, press 1. To quit press 2: ");

        /*
        there are easier ways to do this part below such as using
        let loop_control = input_in(), I just wanted to play around with passing a match value
        into a variable and error checking of .parse()
        */

        let loop_control: u32 = match input_in().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nDid not receive number. Quitting the program...");
                break;
            }
        };
        match loop_control {
            1 => {continue}
            2 => {
                println!("\nQuitting the program...");
                break;
            }
            _ => {
                println!("\nDid not receive a 1 or 2 selection. Quitting the program...");
                break;
            }
        }
    }
}


fn convert_to_celsius(temperature: f64) -> f64{
    return (temperature - 32 as f64) / 1.8;
}

fn convert_to_fahrenheit(temperature: f64) -> f64{
    return (temperature * 1.8) + 32 as f64;
}

fn input_in() -> String {
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input
}
