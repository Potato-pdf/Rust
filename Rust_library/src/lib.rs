use std::ffi::CString;
use std::os::raw::c_char;

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
    let test_sdk = Box::new(TestStruct{
        id,
        request_count: 0,
    });
    Box::into_raw(test_sdk)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_increment_request_count(ptr: *mut TestStruct) {
    if !ptr.is_null(){
        let sdk = &mut *ptr;
        sdk.request_count += 1;
        println!("Request count incremented: {}", sdk.request_count);
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn clean_string(ptr: *mut TestStruct) {
    if !ptr.is_null(){
        drop(Box::from_raw(ptr));
        println!("Memory cleaned for TestStruct");
    }

}
