fn main() {
    let mut rng = rand::thread_rng();
    let dice1: u32 = rng.gen_range(1..=6);
    let dice2: u32 = rng.gen_range(1..=6);
    println!("You rolled a {} and a {}.", dice1, dice2);
}
