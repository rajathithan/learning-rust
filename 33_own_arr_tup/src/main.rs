fn main() {
    let arrboolvals: [bool; 3] = [true, false, true];
    // The bool type implements the Copy trait, so indexing into the array copies the value.
    let arrboolindx: bool = arrboolvals[0];
    println!("The value at index 0 of arrboolvals is {}", arrboolindx);

    let tupboolvals: (bool, bool, bool) = (true, false, true);
    // Tuple elements are accessed using dot notation. Since bool is Copy, the value is copied.
    let tupboolindx: bool = tupboolvals.0;
    println!("The value at index 0 of tupboolvals is {}", tupboolindx);


    let arrstringvals: [String; 3] = [
        String::from("Hello"),
        String::from("World"),
        String::from("Rust"),
    ];
    // String does not implement the Copy trait. Rust prevents moving ownership out of an 
    // array index because it cannot track the "moved" status of individual array elements.
    // let arrstringindx: String = arrstringvals[0]; // ERROR: cannot move out of type `[String; 3]`

    // Instead, we borrow the value at the specific index by using a reference.
    let arrstringindx: &String = &arrstringvals[0];
    println!("The value at index 0 of arrstringvals is {}", arrstringindx);


    let tupstringvals: (String, String, String) = (
        String::from("Hello"),
        String::from("World"),
        String::from("Rust"),
    );
    // Unlike arrays, Rust allows moving ownership out of individual tuple fields. 
    // However, doing so would result in a "partial move," making the original tuple 
    // and the moved field inaccessible for future use.
    
    // This moves ownership of "Hello" from the tuple to the variable.
    // The tuple 'tupstringvals' becomes "partially moved."
    let _tup_moved_val: String = tupstringvals.0; 
    println!("The value at index 0 of tupstringvals is {}", _tup_moved_val);

    // However, if we want to keep the tuple intact and use it later, 
    // we MUST use a reference as you did below:
    // Note that you can't access index 0 as the ownership of that field has been moved to _tup_moved_val.
    let tupstringindx: &String = &tupstringvals.1; // Borrowing index 1
    println!("The value at index 1 of tupstringvals with reference is {}", tupstringindx);

    
}
