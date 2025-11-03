#include <lean/lean.h>
#include <stdlib.h>

extern uint32_t risc0_main(uint32_t input);

uint32_t lean_simple_risc0_main(uint32_t n) {
    return risc0_main(n);
}
