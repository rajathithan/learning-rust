fn main() {
    // A reference is a type of pointer to a value that allows you to access the value without taking ownership of it
    // Generally for stack data types, we can use the copy trait to create a copy of the value, 
    // but for heap data types, we can use references to access the value without taking ownership
    let num: i32 = 5;
    let num_ref: &i32 = &num; // num_ref is a reference to num

    // heap data types do not implement the Copy trait, so we can use references to access the value without taking ownership
    let greeting: String = String::from("Hello, world!");
    let greeting_ref: &String = &greeting; // greeting_ref is a reference to greeting

    // pointers used in other languages may reference a memory location that may or may not be valid, and it can be null or dangling
    // A reference is always valid and cannot be null or dangling, it must always point to a valid value
}
