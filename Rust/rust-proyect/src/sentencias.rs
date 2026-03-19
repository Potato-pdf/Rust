pub fn sentencia() {
    println!("--- Sentencias ---");

    // Sentencia de declaración
    let x = 5;

    // Sentencia de expresión
    let y = {
        let x = 3;
        x + 1
    };

    println!("x: {}, y: {}", x, y);

    // Control de flujo como sentencias/expresiones
    if x < 10 {
        println!("x es menor que 10");
    } else {
        println!("x es mayor o igual que 10");
    }

    let mut count = 0;
    while count < 3 {
        println!("count: {}", count);
        count += 1;
    }

    for i in 0..3 {
        println!("for loop i: {}", i);
    }
}