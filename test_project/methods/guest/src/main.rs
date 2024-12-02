use risc0_zkvm::{guest::env};
mod vec_sum10;
mod vec_sum100;

fn main() {
    let test_name: String = env::read::<String>();
    if test_name == "vecSum10" {
        let result: i32 = vec_sum10::test_func();
        env::commit(&result);
    }
    else if test_name == "vecSum100" {
        let result: i32 = vec_sum100::test_func();
        env::commit(&result);
    }
}