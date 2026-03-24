use std::ffi::CString;
use std::os::raw::c_char;

/**
NO mangle inhibits the name mangling of the function.
This is necessary for the function to be called from C.
**/
#[unsafe(no_mangle)]

pub extern "C" fn mudra_init_handshake() -> *mut c_char {
    let message = "Mudra Core v0.1.0";
    /*
    Convertimos un String de Rust a un CString (compatible con C/Bun)
      */
    let c_message = CString::new(message).unwrap();
    /*
    into_raw() convierte el CString en un puntero crudo a c_char
    Trasnfiere la propiedad de la memoria a el que lo reciba
    */
    c_message.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn mudra_free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s); // Libera la memoria
    }
}
