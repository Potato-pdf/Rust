pub fn datos_escalares() {
    let numero_sin_signo: u32 = 42; // u32 es un tipo de dato que representa un entero sin signo de 32 bits osea del 0 al 4,294,967,295
    println!("Numero sin signo: {}", numero_sin_signo);
    let numero_con_signo: i32 = -42; // i32 es un tipo de dato que representa un entero con signo de 32 bits osea del -2,147,483,648 al 2,147,483,647
    println!("Numero con signo: {}", numero_con_signo);
    let numero_flotante: f32 = 3.14; // f32 es un tipo de dato que representa un numero de punto flotante de 32 bits representa numeros con decimales
    println!("Numero flotante: {}", numero_flotante);
    let booleano: bool = true; // bool es un tipo de dato que representa un valor booleano, puede ser true o false
    println!("Booleano: {}", booleano);
    let caracter: char = 'A'; // char es un tipo de dato que representa un caracter unicode
    println!("Caracter: {}", caracter);
    let emoji: char = '😊'; // char es un tipo de dato que representa un caracter unicode, puede ser un emoji
    println!("Emoji: {}", emoji);   

}