use std::io; //Importacion de la libreria de entrada y salida
use rand::Rng; // Importacion de la libreria rand para generar numeros aleatorios
use std::cmp::Ordering; // Importacion de la libreria cmp para comparar valores

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // Genera un numero aleatorio entre 1 y 100
    println!("Please input your guess.");
    let mut guess = String::new();// Crea una variable mutable llamada guess que es una cadena de texto vacia

    //std::io::stdin se puede utilizar la libreria std:: aun sin importar al principio del codigo
    io::stdin()// Toma la entrada del usuario desde la consola
        .read_line(&mut guess)// Lee la linea de entrada y la guarda en la variable guess, el & permite que varias partes del corigo accedan a una pieza de datos sin sobreescribirla q 
        .expect("Failed to read line");// Si hay un error al leer la linea, imprime el mensaje de error
    println!("You guessed: {secret_number}");

}
