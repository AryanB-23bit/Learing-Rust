use std::io;
use std::io::Write;

fn main() {
    let four = "4";
    let four_int: u32 = four.parse().unwrap();

    let mut temp_sign = String::new();

    let test_str = String::new();

    print!("Type in \"C\" for Celsius or \"F\" for Fahrenheit: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut temp_sign)
        .expect("Failed to read line");


    conversion(test_str, temp_sign.trim().to_uppercase());
    // println!("{}", four_int);
    assert_eq!("4", four);

    println!("{}",temp_sign.trim().to_uppercase())

}

fn conversion(temperature: String, converting_to: String) {
    if converting_to == "C" {
        println!("test completed");
    }
    else {
        println!("{}", converting_to.to_uppercase());
    }
}