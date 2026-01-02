fn main() {
    let a: i32 = 10;
    let b: f64 = a as f64; // Casting i32 to f64
    println!("Integer: {}, as Float: {}", a, b);

    let pi: f64 = 3.14159;
    let pi_int: i32 = pi as i32; // Casting f64 to i32
    println!("Float: {}, as Integer: {}", pi, pi_int);
}
