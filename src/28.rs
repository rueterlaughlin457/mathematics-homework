fn main() {
    let n = 5;
    println!("The sum of natural numbers up to {}: {}", n, calculate_sum(n));
}

fn calculate_sum(n: i32) -> i32 {
    (1..=n).sum()
}
