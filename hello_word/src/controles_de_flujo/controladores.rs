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
pub fn match_example() {
    let numero: i8 = 5;

}