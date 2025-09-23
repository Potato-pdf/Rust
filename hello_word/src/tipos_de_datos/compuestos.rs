pub fn datos_compuestos() {
    let tupple: (i32, f64, u8) = (500, 6.4, 1); // Tupla es un tipo de dato compuesto que puede contener varios tipos de datos inmutables
    let (x, y, z) = tupple; // Desestructura la tupla en variables individuales
    println!("El valor de y es: {}", x); // Imprime el valor de y}

    let arrays: [i32; 5] = [1, 2, 3, 4, 5]; // Array es un tipo de dato compuesto que puede contener varios valores del mismo tipo de dato y tiene un tamaño fijo
    let primer_elemento = arrays[0]; // Accede al primer elemento del array
    println!("El primer elemento del array es: {}", primer_elemento); // Imprime el primer elemento del array
    println!("Segundo elemnto de un array: {}", arrays[1]); // Imprime el segundo elemento del array

    
}