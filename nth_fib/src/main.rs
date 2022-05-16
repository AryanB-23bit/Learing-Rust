fn main() {
    let x:u64 = 30;
    let results = nth_fib(x);
    println!("{}", results)
}

fn nth_fib(user_num: u64)->u64{
    if user_num == 0 {user_num}
    else if user_num == 1 {user_num}
    else {
        nth_fib(user_num-1) + nth_fib(user_num-2)
    }
}
