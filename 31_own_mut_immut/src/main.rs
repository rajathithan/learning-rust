fn main() {
    let coffee_type = String::from("Espresso");
    let a = &coffee_type; // Immutable reference to coffee_type
    
    // Shared references (&T) implement the Copy trait. 
    // This creates a new reference 'b' by copying the pointer 'a'.
    let b = a; 
    
    let _ = &coffee_type; // Another immutable reference created directly
    
    // This works because we can have multiple immutable references simultaneously.
    println!("The type of coffee of a {} and b {}", a, b);


    let mut coffee_type_mut = String::from("Latte");
    let c: &mut String = &mut coffee_type_mut; // Mutable reference to coffee_type_mut

    // Mutable references (&mut T) do NOT implement Copy; they are MOVED.
    // Ownership of the unique access to 'coffee_type_mut' moves from 'c' to 'd'.
    let d: &mut String = c; 

    // ERROR: The following line will fail to compile.
    // 'c' is no longer valid because its reference was moved to 'd'.
    // Rust prevents this to ensure that only one mutable path to the data exists at a time.
    // println!("The type of coffee of c {} and d {}", c, d); 
    
    // This would work if you only used 'd':
    println!("The type of coffee of d is {}", d);
}
