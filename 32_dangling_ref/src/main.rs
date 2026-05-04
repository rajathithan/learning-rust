fn main() {
    // This would fail to compile because it tries to receive a reference 
    // to data that no longer exists.
    // let city1: &String = dangling_create_city(); 

    let city2: String = create_city();
    println!("City 2: {}", city2);
}

/*
fn dangling_create_city() -> &String {
    let city: String = String::from("New York");
    
    // ERROR: 'city' is owned by this function. 
    // When the function ends, 'city' is dropped and its memory is deallocated.
    // Returning &city would be returning a pointer to invalid memory (a dangling pointer).
    &city 
}
*/

fn create_city() -> String {
    let city: String = String::from("Los Angeles");
    
    // SUCCESS: We are returning the 'String' itself, not a reference.
    // Ownership is "MOVED" from this function to the caller in main().
    // Because the ownership moves, the data is not dropped when the function ends.
    city 
}
