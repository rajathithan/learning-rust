fn main() {
    // stack data types implement the Copy trait
    let num: i32 = 5;
    let num_copy: i32 = num; // Copy Trait 

    // Both num and num_copy can be used independently
    // because i32 implements the Copy trait
    println!("num: {}, num_copy: {}", num, num_copy);

    // Heap data types do not implement the Copy trait 
    let greeting: String = String::from("Hello, world!");

    let another_greeting: String = greeting.clone(); // Clone Trait

    // since we used clone, both greeting and another_greeting can be used independently
    println!("greeting: {}, another_greeting: {}", greeting, another_greeting);

    // String ownership is moved from greeting to welcomes
    // greeting can no longer be used after this point
    let welcomes: String = greeting; // Move Trait

    // This line will cause a compile-time error
    // move occurs because `greeting` has type `String`, which does not implement the `Copy` trait
    
    //println!("greeting: {}", greeting);

    drop(welcomes); // Explicitly drop the ownership of welcomes

    // This will also cause a compile-time error
    // value borrowed here after move
    
    //println!("welcomes: {}", welcomes); 

}
