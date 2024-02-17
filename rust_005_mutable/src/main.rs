fn main() {
    // el palabra clave `mut` se permite la variable ser cambiado
    let mut a_number = 10;
    println!("El número es {}.", a_number);

    // Cambio el valor de una variable mutable
    a_number = 15;
    println!("Ahora el número es {}.", a_number);
}
