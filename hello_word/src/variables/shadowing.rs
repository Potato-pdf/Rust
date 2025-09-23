pub fn shadowing() {
    let numero: i128 = 5;
    let numero: i128 = numero + 1; // Sombreamos la variable numero, ahora numero es 6
    print!("Numero sombreador: {}", numero);// sombrear una variable, quiere decir declarar una nueva variable con el mismo nombre que una variable anterior, la nueva variable "sombrea" a la anterior
}