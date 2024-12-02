use methods::{
    TEST_PROJECT_ELF, TEST_PROJECT_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::env;
mod benchmarker;
use bincode;

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

    //starts time and peak memory benchmark measurements, proves test project, then ends benchmarks measurements
    let mut benchmarker = benchmarker::Benchmarker::new();
    benchmarker.start_benchmark();
    let prove_info = prover
        .prove(env, TEST_PROJECT_ELF)
        .unwrap();
    let benchmark_results = benchmarker.end_benchmark();

    //log proving benchmark results
    if let Some((duration, peak_memory)) = benchmark_results {
        println!("Proving time: {:?}", duration);
        println!("Peak memory consumption during proving: {} KB", peak_memory);
    }

    //take out the receipt
    let receipt = prove_info.receipt;

    //serialize the receipt to its bytes and log its size in kb
    let serialized_receipt = bincode::serialize(&receipt).unwrap();
    let size_in_kb = serialized_receipt.len() as f64 / 1024.0;
    println!("Proof size: {} KB", size_in_kb);

    //starts time and peak memory benchmarks measurements, verifies the receipt, then ends benchmarks measurements
    let mut verifying_benchmarker = benchmarker::Benchmarker::new();
    verifying_benchmarker.start_benchmark();
    receipt
        .verify(TEST_PROJECT_ID)
        .unwrap();
    let verifying_benchmark_results = verifying_benchmarker.end_benchmark();

    //logs verification benchmark results
    if let Some((duration, peak_memory)) = verifying_benchmark_results {
        println!("Verification time: {:?}", duration);
        println!("Peak memory consumption during verification: {} KB", peak_memory);
    }
}