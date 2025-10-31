# Lean 4 in RISC Zero Guest

## Running

1. Make sure you have the following environment variables set.
- `LEAN_RISC0_PATH`: path to the Lean RISC0 runtime, typically `$HOME/.lean-risc0`.
- `RISC0_TOOLCHAIN_PATH`: path to RISC0 toolchain, typically `$HOME/.risc0/toolchains/v2024.1.5-cpp-x86_64-unknown-linux-gnu/riscv32im-linux-x86_64`.
2. Install [Lean RISC0 runtime](https://github.com/anoma/lean-risc0-runtime).
3. Install [Lean RISC0 Init standard library](https://github.com/anoma/lean-risc0-init).
4. `cargo build --release`
5. `cargo run --release`
