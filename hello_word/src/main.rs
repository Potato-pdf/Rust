use std::string;

fn main() {
    // Print "Hello, world!" to the console
    println!("Hello, world!");
    // variable 
    let string: &str = "String variable"; // &str es un tipo de dato que representa una cadena de texto que reserva el espacio en memoria
    println!("Variable: {}", string); // Imprime la variable, requiere el uso de {} para indicar que se va a imprimir una variable

    let mut string_mutable: &str = "Cadena mutable"; // &str es un tipo de dato que representa una cadena de texto mutable
    println!("Variable mutable antes del cambio: {}", string_mutable); // Imprime la variable mutable antes del cambio
    string_mutable = "Cadena mutable cambiada"; // Cambia el valor de la variable mutable
    println!("Variable mutable despues del cambio: {}", string_mutable); // Imprime la variable mutable despues del cambio
    // string_mutable = 5; // Error, no se puede cambiar el tipo de dato de una variable

    let string_string: String = String::from("String variable con String"); // String es un tipo de dato que representa una cadena de texto que no reserva el espacio en memoria, solo usa lo que necesita
    println!("Variable con String: {}", string_string); // Imprime la variable con String

    let int: i32 = 5; // i32 es un tipo de dato que representa un entero de 32 bits
    println!("Variable entera: {}", int); // Imprime la variable entera
    let 
}