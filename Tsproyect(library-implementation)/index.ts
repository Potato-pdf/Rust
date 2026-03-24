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
    mudra_init_handshake: {
        args: [],
        returns: FFIType.ptr,
    },
    mudra_free_string: {
        args: [FFIType.ptr],
        returns: FFIType.void,
    }

})

/**
 * Llamamos a la funcion mudra_init_handshake
 */
console.log("Conectando con Mudra Core...");
const responsePtr = lib.symbols.mudra_init_handshake();
/**
 * Validamos que el puntero no sea nulo
 */
if (!responsePtr) {
    console.error("Error: mudra_init_handshake returned null pointer");
} else {
    const response = new CString(responsePtr);
    console.log(response.toString());
    /**
     * Liberamos la memoria
     */
    lib.symbols.mudra_free_string(responsePtr);
}