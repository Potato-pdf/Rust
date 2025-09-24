use crate::funciones;

pub fn funcion_ejemplo() {
    let param1: i32 = 10;
    let param2: i32 = 20;
    let resultado1 = funciones::suma(param1, param2);
    resultado1
}

pub fn suma (a: i32, b: i32) -> i32 {
    a + b
}