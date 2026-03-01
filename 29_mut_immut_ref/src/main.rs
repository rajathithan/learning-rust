
// This code demonstrates the use of mutable and immutable references in Rust.


// The add_fries and add_coke functions take mutable references to the meal String, 
// allowing them to modify it without taking ownership.
// you cannot use immutable references to modify the value, 
// as you cannot call a method that modifies the value through an immutable reference.
fn add_fries(meal: &mut String) {
    meal.push_str(" with fries");
}

fn add_coke(meal: &mut String) {
    meal.push_str(" and a coke");
}

// The show_food function takes an immutable reference to the meal String, 
// allowing it to read the value without modifying it.
fn show_food(meal: &String) {
    println!("{}", meal);
}


fn main() {
    let mut meal = String::from("Burger");
    // We can pass a mutable reference to the meal String to the functions
    add_fries(&mut meal);
    add_coke(&mut meal);

    // Now we can show the final meal using an immutable reference
    show_food(&meal);
}
