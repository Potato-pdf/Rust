/*!
Las funciones son bloques de código reutilizables.
En Rust las funciones se definen con la palabra clave `fn`.
Se pueden pasar parámetros y devolver valores indicando el tipo con "->".
*/

/**
Simplemente imprime un saludo en la consola.
*/
pub fn saludar() {
    println!("¡Hola!");
}

/**
Suma dos enteros de 32 bits y devuelve el resultado.
Rust devuelve la última expresión de una función de forma implícita si no termina en punto y coma.
*/
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

/**
Resta dos enteros de 32 bits y devuelve el resultado.
*/
pub fn restar(a: i32, b: i32) -> i32 {
    a - b
}


