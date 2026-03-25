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
  test_constructor: { args: [FFIType.u32], returns: FFIType.ptr },
  test_increment_request_count: { args: [FFIType.ptr], returns: FFIType.u32 }, // devuelve el nuevo contador
  test_get_request_count: { args: [FFIType.ptr], returns: FFIType.u32 },
  test_free: { args: [FFIType.ptr], returns: FFIType.void },
});
console.log(Object.keys(lib), Object.keys(lib.symbols)); // inspecciona lo cargado

/**
 * usamos la libreria para crear una instancia de TestStruct y luego incrementamos el contador de peticiones
 */
const ptr = lib.symbols.test_constructor(102);
let interval = setInterval(() => {
    const newCount = lib.symbols.test_increment_request_count(ptr);
    console.log(`Request count incremented: ${newCount}`);
    if (newCount >= 1000000) {
        clearInterval(interval);
        console.log("Reached 1000000 requests, stopping increments.");
    }
}, 0);



/**
 * Limpiamos la memoria asignada para TestStruct
 */
lib.symbols.test_free(ptr);