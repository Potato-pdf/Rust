pub fn if_example() {
        let numero: i8 = 5;
        if numero < 10 {
            println!("El número es menor que 10");
        }
}
pub fn else_example() {
    let numero: i8 = 5;

    if numero < 10 {
        println!("El número es menor que 10");
    } else {
        println!("El número es mayor o igual que 10");
    }
}
pub fn else_if_example() {
    let numero: i8 = 5;

    if numero < 10 {
        println!("El número es menor que 10");
    } else if numero == 10 {
        println!("El número es igual a 10");
    } else {
        println!("El número es mayor que 10");
    }
}
pub fn match_example() { // Es switch pero más potente y mas estricto
    let numero: i8 = 5;
    match numero {
        1 => println!("El número es 1"),
        2 => println!("El número es 2"),
        3 => println!("El número es 3"),
        4 => println!("El número es 4"),
        5 => println!("El número es 5"),
        _ => println!("El número no está entre 1 y 5"), // El guion bajo es un comodín que captura cualquier valor no especificado
    }
}