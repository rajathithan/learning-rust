fn main() {
    
    // stack are memory allocated for fixed size data types
    // heap are memory allocated for dynamic size data types (which can grow and shrink in size)

    // String type that is not stored in heap or stack, directly stored in binary during compile time
    // These are immutable and have a fixed size
    let str_literal: &str = "Hello, world!";
    println!("String Literal: {}", str_literal);

    // String new creates an empty String type
    // the text variable is allocated on the heap but has zero length initially
    // it takes the ownership of deallocating memory when it goes out of scope
    let text : String = String::new();

    // String type that is stored in heap memory, can grow and shrink in size
    let mut heap_string: String = String::from("Hello");
    heap_string.push_str(", world!");
    println!("Heap String: {}", heap_string);

}
