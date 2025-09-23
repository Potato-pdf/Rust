pub fn operadores_aritmeticos() {
    let a: i8 = 10;
    let b: i8 = 5;
    print!("Suma: {}", a + b); // Suma
    print!("Resta: {}", a - b); // Resta
    print!("Multiplicacion: {}", a * b); // Multiplicacion
    print!("Division: {}", a / b); // Division, unicamente en enteros
    print!("Modulo: {}", a % b); // Modulo, unicamente en enteros
}

pub fn operadores_logicos() {
    let a: bool = true;
    let b: bool = false;
    print!("AND: {}", a && b); // AND
    print!("OR: {}", a || b); // OR
    print!("NOT: {}", !a); // NOT
}

pub fn operadores_de_comparacion() {
    let a = 10;
    let b = 5;
    print!("Igual: {}", a == b); // Igual
    print!("No igual: {}", a != b); // No igual
    print!("Mayor que: {}", a > b); // Mayor que
    print!("Menor que: {}", a < b); // Menor que
    print!("Mayor o igual que: {}", a >= b); // Mayor o igual que
    print!("Menor o igual que: {}", a <= b); // Menor o igual que
}