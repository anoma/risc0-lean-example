# Lean 4 in RISC Zero Guest

## Running

1. Make sure you have the following environment variables set.
- `LEAN_RISC0_PATH`: path to the Lean RISC0 runtime, typically `$HOME/.lean-risc0`.
- `RISC0_TOOLCHAIN_PATH`: path to RISC0 toolchain, typically `$HOME/.risc0/toolchains/v2024.1.5-cpp-x86_64-unknown-linux-gnu/riscv32im-linux-x86_64`.
2. Install [Lean RISC0 runtime](https://github.com/anoma/lean-risc0-runtime).
3. Install [Lean RISC0 Init standard library](https://github.com/anoma/lean-risc0-init).
4. `just build`
5. `target/release/host N`

## Main example

The `main` branch contains an example of a `sum` function in Lean operating on `Nat`. The example implements a general interface to Lean 4, passing data via a byte array which is then parsed to `Nat` on the Lean side. The result is returned in a byte array which is then parsed on the Rust side. The example properly initializes the runtime.

## Sum example

The `sum-example` branch contains a lightweight example of a `sum` function in Lean operating on 32-bit unsigned integers. This example doesn't perform runtime initialization.
