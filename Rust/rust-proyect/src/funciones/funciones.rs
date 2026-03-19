/*
Las funciones son bloques de código reutilizables.
En Rust las funciones se definen con la palabra clave `fn`.
Se pueden pasar parámetros y devolver valores indicando el tipo con "->".
*/

pub fn saludar() {
    /*
    Simplemente imprime un saludo en la consola.
    */
    println!("¡Hola!");
}

pub fn sumar(a: i32, b: i32) -> i32 {
    /*
    Suma dos enteros de 32 bits y devuelve el resultado.
    Rust devuelve la última expresión de una función de forma implícita si no termina en punto y coma.
    */
    a + b
}

pub fn restar(a: i32, b: i32) -> i32 {
    /*
    Resta dos enteros de 32 bits y devuelve el resultado.
    */
    a - b
}

