#include <lean/lean.h>

extern lean_object* risc0_main(lean_object* input);
extern lean_object* initialize_Guest(uint32_t builtin, lean_object* world);
extern void lean_initialize_runtime_module();

static lean_object* byte_array_from_c(const char* buf, size_t n) {
    lean_object* ba = lean_mk_empty_byte_array(lean_usize_to_nat(n));
    for (size_t i = 0; i < n; ++i) {
        ba = lean_byte_array_push(ba, (uint32_t)(uint8_t)buf[i]);
    }
    return ba;
}

static char* c_from_byte_array(lean_object* ba, size_t* out_n, int want_nul) {
    size_t n = (size_t)lean_byte_array_size(ba);
    size_t m = n + (want_nul ? 1 : 0);
    char* out = (char*)malloc(m);
    if (!out) {
        return NULL;
    }
    for (size_t i = 0; i < n; ++i) {
        out[i] = (char)(unsigned char)lean_byte_array_uget(ba, i);
    }
    if (want_nul) {
        out[n] = '\0';
    }
    if (out_n) {
        *out_n = n;
    }
    return out;
}

int lean_risc0_main(const char* src, size_t src_size, char** dst, size_t* dst_size) {
    lean_initialize_runtime_module();
    lean_object* res = initialize_Guest(1, lean_io_mk_world());
    // if (!lean_io_result_is_ok(res)) {
    //     lean_dec_ref(res);
    //     return 1;
    // }
    lean_dec_ref(res);
    lean_object* in = byte_array_from_c(src, src_size);
    lean_object* out = risc0_main(in);
    *dst = c_from_byte_array(out, dst_size, 0);
    lean_dec_ref(out);
    return 0;
}
