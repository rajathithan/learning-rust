fn main() {

    // pointers used in other languages may reference a memory location that may or may not be valid, and it can be null or dangling
    // A reference is always valid and cannot be null or dangling, it must always point to a valid value


    // A reference is a type of pointer to a value that allows you to access the value without taking ownership of it
    // Generally for stack data types, we can use the copy trait to create a copy of the value, 
    // but for heap data types, we can use references to access the value without taking ownership
    let num: i32 = 5;
    let num_ref: &i32 = &num; // num_ref is a reference to num

    // Deferencing a reference allows us to access the value it points to
    println!("The value of num is: {}", *num_ref); // dereferencing num_ref to access the value of num

    // Unlike other languages, you can still get the value of a reference without dereferencing it,
    // because Rust automatically dereferences references when needed using the display trait
    println!("The value of num is: {}", num_ref); // Rust automatically dereferences num

    // heap data types do not implement the Copy trait, so we can use references to access the value without taking ownership
    let greeting: String = String::from("Hello, world!");
    let greeting_ref: &String = &greeting; // greeting_ref is a reference to greeting

    println!("The value of greeting is: {}", *greeting_ref); // dereferencing greeting_ref to access the value of greeting
    println!("The value of greeting is: {}", greeting_ref); // Rust automatically dereferences greeting_ref to access the value of greeting

    // References can be used to create multiple references to the same value without taking ownership
    let apples : &str = "fruit";
    let oranges : &str = apples;
    println!("apple is a {} and orange is also a {}" , apples, oranges);
}
    

