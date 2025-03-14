
use std::num::NonZeroU32;

fn main() {
    let mut rng = rand::thread_rng();
    let non_zero_u32: NonZeroU32 = NonZeroU32::new(rng.gen()).unwrap();
    println!("{}", non_zero_u32);
}