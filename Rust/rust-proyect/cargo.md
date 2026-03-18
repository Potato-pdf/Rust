structura que se creó:

    Cargo.toml: Es el manifiesto de tu proyecto (equivalente al package.json). Aquí defines el nombre, la versión y las dependencias.

    src/main.rs: Aquí vive tu código fuente. Cargo ya te puso un "Hello World" ahí.

El flujo de trabajo profesional

Olvida rustc. De ahora en adelante, usarás estos tres comandos:

    cargo check: El comando más usado. Comprueba si tu código compila, pero no genera un binario. Es rapidísimo y es lo que usas mientras programas para ver si tienes errores.

    cargo build: Compila el proyecto y genera un ejecutable en target/debug/.

    cargo run: Compila (si hubo cambios) y ejecuta el programa en un solo paso.

    cargo build --check: Compila y optimiza el codigo
    
