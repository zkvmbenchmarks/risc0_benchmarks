use risc0_zkvm::{guest::env};
mod vecSum10;

fn main() {
    let test_name: String = env::read::<String>();
    if test_name == "vecSum10" {
        let result: i32 = vecSum10::test_func();
        env::commit(&result);
    }
}