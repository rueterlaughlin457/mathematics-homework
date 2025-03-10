use std::{cmp::Ordering, io};

fn main() {
    let num1 = 5;
    let num2 = 8;

    match num1.cmp(&num2) {
        Ordering::Less => println!("{} is less than {}", num1, num2),
        Ordering::Greater => println!("{} is greater than {}", num1, num2),
        Ordering::Equal => println!("{} is equal to {}", num1, num2),
    }

    // arithmetic operations
    let sum = num1 + num2;
    let diff = num1 - num2;
    let prod = num1 * num2;
    let quot = num1 / num2;
    let rem = num1 % num2;

    println!("sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}", sum, diff, prod, quot, rem);
}
