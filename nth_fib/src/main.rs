use std::io;
use std::io::Write;

fn main() {
    println!("Please enter a number and program will give you that Fibonacci number.\n");

    let num = input_int();
    let results = nth_fib(num);
    println!("Fibonacci(n={}) = {}", num, results)
}

fn nth_fib(user_num: u64)->u64{
    if user_num == 0 {user_num}
    else if user_num == 1 {user_num}
    else {
        nth_fib(user_num-1) + nth_fib(user_num-2)
    }
}

fn input_int() -> u64 {
    loop {
        let mut user_input = String::new();
        
        print!("Please keep the number low as the program is using recursion, it will take a lot of time if the number is high: ");
        
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut user_input).expect("Failed to read line");
        
        let user_input:u64 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) =>{println!("\nDid not receive number. Restarting the program...\n");
                      continue }
        };
        return user_input;
    }
}
