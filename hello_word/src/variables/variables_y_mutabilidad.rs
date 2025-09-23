pub fn variables_y_mutabilidad() {
    let numero_inmutable: i32 = 43; // Variable inmutable, let es la palabra reservada para declarar variables
    println!("Numero inmutable: {}", numero_inmutable);

    let mut numero_mutables: i32 = 44; // Variable mutable, mut es la palabra reservada para convertir una variable en mutable
    println!("Numero mutable antes del cambio: {}", numero_mutables);
    numero_mutables = 45;
    println!("Numero mutable después del cambio: {}", numero_mutables);

    const CONSTANTE: i32 = 100; // Constante, const es la palabra reservada para declarar constantes, las constantes deben tener un tipo de dato y no pueden cambiar su valor
    println!("Constante: {}", CONSTANTE);
}