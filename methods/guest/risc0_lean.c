#include <lean/lean.h>
#include <stdlib.h>

extern uint32_t risc0_main(uint32_t input);
extern lean_object* initialize_Guest(uint32_t builtin, lean_object* world);
extern void lean_initialize_runtime_module();

uint32_t lean_simple_risc0_main(uint32_t n) {
    lean_initialize_runtime_module();
    lean_object* res = initialize_Guest(1, lean_io_mk_world());
    if (!lean_io_result_is_ok(res)) {
        lean_dec_ref(res);
        return 0;
    }
    lean_dec_ref(res);
    return risc0_main(n);
}
