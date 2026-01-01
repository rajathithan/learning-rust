fn main() {
    let pi: f64 = 3.141592653589793;
    println!("The value of pi is approximately: {}", pi);
    println!("Pi rounded to two decimal places: {:.2}", pi);
    println!("Pi rounded up (ceil): {}", pi.ceil());
    println!("Pi rounded down (floor): {}", pi.floor());
    println!("Pi rounded to the nearest integer: {}", pi.round());
    println!("Pi in scientific notation: {:e}", pi);
}
