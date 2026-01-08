fn main() {
    
    // Generic is used for defining arguments and return types of functions, structs, enums, and traits.
    // For the below example, we are using std::ops::Range and std::ops::RangeInclusive to define ranges.
    let monthdays_exclusive: std::ops::Range<i32> = 1..31;
    let monthdays_inclusive: std::ops::RangeInclusive<i32> = 1..=31;

    println!("Days in a month (exclusive range): {:?}", monthdays_exclusive);
    println!("Days in a month (inclusive range): {:?}", monthdays_inclusive);

    for day in monthdays_exclusive {
        print!("{} ", day);
    }
    println!();

    for day in monthdays_inclusive {
        print!("{} ", day);
    }
    println!();

    // Iterating over ranges
    let characters: std::ops::Range<char> = 'a'..'e';
    for c in characters {
        println!("Character: {}", c);   
    }

    println!();

     // Iterating over ranges
    let characters: std::ops::RangeInclusive<char> = 'a'..='e';
    for c in characters {
        println!("Character: {}", c);   
    }

}
