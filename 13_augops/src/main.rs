fn main() {
    let mut score: u32 = 85;
    println!("Original Score: {}", score);

    // Add 5 bonus points
    score += 5;
    println!("After adding 5 bonus points: {}", score);

    // Minus 3 penalty points
    score -= 3;
    println!("After subtracting 3 penalty points: {}", score);

    // Double the score
    score *= 2;
    println!("After doubling the score: {}", score);

    // Halve the score
    score /= 2;
    println!("After halving the score: {}", score);
}
