/*
El ownership es un concepto fundamental en Rust que permite gestionar la memoria de forma segura sin necesidad de un recolector de basura.
El scope es lo que esta dentro de las llaves {} delimita el tiempo de vida de una variable
Cuando una variable sale de su scope, se elimina de la memoria
*/

pub fn ownership() {
    /*
    en este caso, "saludo" es el dueño de "hola" y cuando sale de su scope, se elimina de la memoria
    */
    let saludo = String::from("hola");
    println!("{}", saludo);
    
    /*
    en este caso, "mensaje" es el dueño de "hola" no se rompe la propiedad de un solo dueño, 
    si trataramos de hacer println!("{}", saludo); daria error porque saludo ya no es dueño de "hola"
    */
    let mensaje = saludo;
    println!("{}", mensaje);
}
