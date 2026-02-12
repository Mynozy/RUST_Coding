fn main() {
    println!("Hello, world!");

    let a: i32 = 10;
    let b: i32 = 3;

    println!("═══════════════════════════");
    println!("  CALCULADORA SIMPLE");
    println!("═══════════════════════════");
    println!();
    println!("Numero 1: {}", a);
    println!("Numero 2: {}", b);
    println!();
    println!("Suma:  {} + {} = {}", a, b, a+b);
    println!("Resta: {} - {} = {}", a, b, a-b);
    println!("División: {} / {} = {}", a, b, a/b);
    println!("Multiplicación: {} * {} = {}", a, b, a*b);
    println!("Mod: {} % {} = {}", a, b, a%b);
}