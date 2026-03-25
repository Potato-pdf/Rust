// Eliminados imports no usados

/** 
Structura de prueba privada para proyectos fuera de Rust_library.
*/
pub struct TestStruct {
    id: u32,
    request_count: u32,
}
/**
Constructor de prueba para TestStruct.
Crea una nueva instancia de TestStruct con el id proporcionado y un request_count inicial de 0.
Utilizamos "Box::" para asignar la estructura en el heap, lo que permite que su memoria persista después de que la función termine.
con "Box::into_raw" convertimos el Box en un puntero crudo, transfiriendo la propiedad de la memoria al llamador (en este caso, Bun).
*/
#[unsafe(no_mangle)]
pub extern "C" fn test_constructor(id: u32) -> *mut TestStruct {
    let s = Box::new(TestStruct { id, request_count: 0 });
    Box::into_raw(s)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_increment_request_count(ptr: *mut TestStruct) -> u32 {
    if ptr.is_null() { return 0; }
    unsafe {
        let s = &mut *ptr;
        s.request_count = s.request_count.saturating_add(1);
        s.request_count
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_get_request_count(ptr: *const TestStruct) -> u32 {
    if ptr.is_null() { return 0; }
    unsafe { (&*ptr).request_count }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_free(ptr: *mut TestStruct) {
    if ptr.is_null() { return; }
    unsafe { let _ = Box::from_raw(ptr); } // libera
}