use risc0_zkvm::{guest::env};

fn main() {
    let test_name: String = env::read::<String>();
    if test_name == "sum10" {
        let input = initialize_large_array(10);
        let sum: i32 = input.iter().sum();
        env::commit(&sum);
    }
}

fn initialize_large_array(size: usize) -> Vec<i32> {
    (0..size as i32).rev().collect()
}