mod funciones;
use funciones::*;
mod expresiones;
use expresiones::*;

fn main() {
    saludar();
    let resultado = sumar(5, 3);
    println!("El resultado de la suma es: {}", resultado);

    let resultado_resta = restar(10, 4);
    println!("El resultado de la resta es: {}", resultado_resta);

    expresiones();

}
