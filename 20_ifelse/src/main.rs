fn main() {
    let number = 15;

    // Using if, else if, and else to check multiple conditions
    if number % 10 == 0 {
        println!("The number is divisible by 10.");
    } else if number % 5 == 0 {
        println!("The number is divisible by 5.");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2.");
    } else {
        println!("The number is not divisible by 10, 5, or 2.");
    }


    // In rust, if is also an expression, which means it returns a value.
    // you can use it on the right side of a let statement to assign a value based on a condition.
    let ans = if number % 2 == 0 {true} else {false};
    println!("Is the number even? {}", ans);

}
