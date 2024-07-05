use rayon::prelude::*;
use num::{BigUint, one, One};
use std::time::Instant;//measure the time it takes to run a function


fn factorial(num : u32)-> BigUint{
    if num==0 || num == 1 {
        return BigUint::one()
    }
    else {
        (1..=num).map(BigUint::from).reduce(|acc, x| acc * x ).unwrap()
    }
}

fn fibonacci_recursive(num : u32) -> u32{
    if num <2{
        num
    }
    else {
        fibonacci_recursive(num-1) +fibonacci_recursive(num -2)
    }
}

fn fibonacci_join(num :u32)-> u32{
    if num < 2{
        num
    }
    else {
        let (a, b) = rayon::join(|| fibonacci_recursive(num-1), || fibonacci_recursive(num-2));
        a  + b
    }
}

fn multithread_factorial(num: u32) -> BigUint{
    if num==0 || num == 1 {
        return BigUint::one()
    }
    else {
        //into parallel iterator
         //this reduce differ from the previous reduce
        (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one(), |acc, x| acc*x)
    }
}

//Rayon help to parallelize the program with garantee from data races
fn main() {
    /*let now = Instant::now();
    println!("{}", factorial(50000));
    println!("{:.2?}", now.elapsed()) ;
    let now = Instant::now();
    println!("{}", multithread_factorial(50000));
    println!("{:.2?}", now.elapsed()) ;
*/
    let now = Instant::now();
    println!("{}", fibonacci_recursive(47));
    println!("{:.2?}", now.elapsed());


    let now = Instant::now();
    println!("{}", fibonacci_join(47));
    println!("{:.2?}", now.elapsed());
}
