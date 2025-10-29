use methods::METHOD_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input: u32 = if args.len() > 1 {
        args[1].parse().expect("Please provide a valid number")
    } else {
        eprintln!("Error: Please provide a number as an argument");
        std::process::exit(1);
    };

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Prove the execution of the guest ELF
    let prove_info = default_prover().prove(env, METHOD_ELF).unwrap();
    let receipt = prove_info.receipt;

    // Read the public result back out of the journal
    let result: u32 = receipt.journal.decode().unwrap();
    println!("Result for {input} is {result}");
}
