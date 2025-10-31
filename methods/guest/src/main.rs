#![no_main]
risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;

mod malloc;

extern "C" {
    fn lean_simple_risc0_main(n: u32) -> u32;
}

fn main() {
    let input: u32 = env::read();

    let value: u32 = unsafe { lean_simple_risc0_main(input) };

    env::commit(&value);
}
