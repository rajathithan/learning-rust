// This example demonstrates passing ownership of a String to functions that modify it
// You need to use mut to modify the String inside the function
fn add_fries(mut meal: String) -> String {
    meal.push_str(" with fries");
    meal
}

// This function takes ownership of the meal String, adds a coke to it, and returns it
fn add_coke(mut meal: String) -> String{
    meal.push_str(" and a coke");
    meal
}


fn main() {
    // Creating a meal by chaining functions that take ownership of the String
    let meal = String::from("Burger");

    // Each function takes ownership of the meal, modifies it, and returns it
    let fries_meal = add_fries(meal);

    // Now completed_meal takes ownership of fries_meal, which is the modified meal with fries
    let completed_meal = add_coke(fries_meal);
    println!("{}", completed_meal);
}
