/*
En Rust, las sentencias son bloques fundamentales de ejecución.
Pueden ser declaraciones de variables o expresiones que terminan con un punto y coma.
*/

pub fn sentencia() {
    /*
    Sentencia de declaración: asocia un valor a un nombre de variable (`let`).
    */
    let x = 5;

    /*
    Sentencia de expresión: Rust permite que los bloques `{}` devuelvan valores.
    En este caso, `y` toma el valor de la última expresión dentro del bloque.
    */
    let y = {
        let x = 3;
        x + 1 // No llevar punto y coma lo convierte en la expresión de retorno del bloque
    };

    println!("x: {}, y: {}", x, y);

    /*
    Control de flujo: `if` también es una expresión en Rust.
    */
    if x < 10 {
        println!("x es menor que 10");
    } else {
        println!("x es mayor o igual que 10");
    }

    /*
    Loops: Rust cuenta con `while` y `for`.
    `for` es ideal para recorrer rangos como `0..3`.
    */
    let mut count = 0;
    while count < 3 {
        println!("count: {}", count);
        count += 1;
    }

    for i in 0..3 {
        println!("for loop i: {}", i);
    }
}