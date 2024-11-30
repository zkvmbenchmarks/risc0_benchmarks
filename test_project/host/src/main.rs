use methods::{
    TEST_PROJECT_ELF, TEST_PROJECT_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::env;
mod benchmarker;
use bincode;
use std::mem;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let input = env::var("TEST_NAME").expect("TEST_NAME environment variable not set");

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let mut benchmarker = benchmarker::Benchmarker::new();
    benchmarker.start_benchmark();

    let prove_info = prover
        .prove(env, TEST_PROJECT_ELF)
        .unwrap();

    let benchmark_results = benchmarker.end_benchmark();

    let receipt = prove_info.receipt;

    let serialized_receipt = bincode::serialize(&receipt).unwrap();
    let size_in_bytes = serialized_receipt.len();
    println!("Size of receipt in bytes: {}", size_in_bytes);

    let _output: u32 = receipt.journal.decode().unwrap();
    println!("{} results: {}", input, _output); // Example output

    if let Some((duration, peak_memory)) = benchmark_results {
        println!("Time it took to prove: {:?}", duration); // Example output
        println!("Peak memory consumption: {} KB", peak_memory); // Example output
    } else {
        println!("Benchmarking data not available");
    }

    let mut verifying_benchmarker = benchmarker::Benchmarker::new();
    verifying_benchmarker.start_benchmark();
    receipt
        .verify(TEST_PROJECT_ID)
        .unwrap();
    let verifying_benchmark_results = verifying_benchmarker.end_benchmark();
    if let Some((duration, peak_memory)) = verifying_benchmark_results {
        println!("Time it took to verify: {:?}", duration); // Example output
        println!("Peak memory consumption during verification: {} KB", peak_memory); // Example output
    } else {
        println!("Benchmarking data not available");
    }
}