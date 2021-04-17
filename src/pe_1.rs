/*
*
*    If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
*
*    Find the sum of all the multiples of 3 or 5 below 1000.
*/



pub fn main() {
    //let mut array = vec![3,5];
    let mut result = 8; // result starts of at 8; 3 + 5 = 8
    //let mut n = 2;
    let limit = 999;
    for n in 6 .. limit {
        if n % 3 == 0 || n % 5 == 0 {
            result +=n;
        }
    }
    println!("Final result is {}", result);
}
