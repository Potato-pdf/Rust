pub fn bucle_loop(){
    let mut contador = 0 ;
    loop {
        println!("El contador es: {}", contador);
        contador += 1;
        if contador == 10 {
            break;
        }
    }
    println!("El bucle ha terminado");
}
pub fn bucle_while(){
    let mut contador = 0 ;
    while contador <10  {
        print!("El contador es: {}", contador);
        contador += 1;
    }
    println!("El bucle ha terminado");
}
pub fn bucle_for(){
    for numero in 0..10 {
        println!("El número es: {}", numero);
    }
    println!("El bucle ha terminado");  

}