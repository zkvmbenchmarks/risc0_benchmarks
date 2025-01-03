use nalgebra::Matrix2;
use risc0_zkvm::guest::env;

fn main() {
    let iterations: u32 = env::read();
    let answer = fibonacci(iterations);
    env::commit(&answer);
}

fn fibonacci(n: u32) -> u64 {
    Matrix2::new(1, 1, 1, 0).pow(n - 1)[(0, 0)]
}