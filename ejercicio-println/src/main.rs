fn main() {
    println!("Hello, world!");

    println!("Hola Rust!");

    //Inline Vars
    println!("Mi edad es {}", 27);
    println!("{} + {} = {}", 5,3, 5+3);

    //Multiple Vals
    println!("Nombre {}, Edad: {}, Ciudad: {}", "Mynor", 27, "Madrid");

    //Specifc Pos
    println!("{0} es de {1} y {0} tiene {2} años", "Mynor", "Costa Rica", 27);

    //Argumentos Named
    println!("{nombre} vive en {ciudad}", nombre="Mynor", ciudad="Madrid");

    //Formato de debugg
    println!("{:?}", (1, 2, 3));

}