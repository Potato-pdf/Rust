import { dlopen, FFIType, ptr, CString } from "bun:ffi";
import { join } from "path";
/**
 * Importamos la libreria de mudra-core recien creada en Rust
 */
const libPath = join(import.meta.dir, "../Rust_library/target/release/libtest_lib.so");

/**
 * Cargamos la libreria en memoria
 * Convertimos el puntero de C a un String de TypeScript
 */
const lib = dlopen(libPath, {
    test_constructor: {
        args: [FFIType.u32],
        returns: FFIType.ptr,
    },
    test_increment_request_count: {
        args: [FFIType.ptr],
        returns: FFIType.void,
    },
    clean_string: {
        args: [FFIType.ptr],
        returns: FFIType.void,
    }

});

