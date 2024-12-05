use risc0_zkvm::{guest::env};

fn main() {
    let mut big_input: Vec<i32> = env::read();
    let sum: i32 = big_input.iter().sum();
    env::commit(&sum);
}
