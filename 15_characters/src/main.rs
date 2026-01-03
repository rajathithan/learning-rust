fn main() {
    // Character variables are defined using single quotes
    // Rust uses the 'char' type to represent a single Unicode scalar value
    // which can be a letter, number, symbol, or even an emoji.
    // Rust uses 4 bytes to store a 'char', allowing it to represent a wide range of characters.

    let _charone: char = 'A';


    let woke: char = '🥱';
    let coffee: char = '☕';
    let work: char = '💼';
    let go: char = '➡';

    println!("I woke up {0} {3}had my {1} {3}went to {2}", woke, coffee, work, go);
}
