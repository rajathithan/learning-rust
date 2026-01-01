fn main() {
    // variable shadowing example
    let gms_of_protein: &str = "50.2344";

    // shadowing the previous variable
    let gms_of_protein: f64 = 50.2344;

    // shadowing again to convert to integer
    let mut gms_of_protein: i32 = 50;

    // modifying the mutable variable
    gms_of_protein = 100;

    println!("I need {} gms of protein", gms_of_protein);

}
