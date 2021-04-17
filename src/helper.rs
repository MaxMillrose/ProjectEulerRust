/*
 * Helper - includes various functions useful for other scripts.
 + Recurring stuff like fibonacci numbers,
 * */

pub fn fib(n: u32) -> u32 {
    //println!("{}",n);
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n-2) + fib(n-1)
    }   // end of match statement
} //end of fib()
