// functions that take parameters and returns string
fn apply_to_jobs(number: i32, title: &str) -> String {
    format!("Applying to job number {}: {}", number, title)
}

// function that returns a boolean
// notice that there is no semicolon or the return keyword
fn is_even(number: i32) -> bool {
    number % 2 == 0
}

// function that returns a tuple of booleans
fn alphabets(s: &str) -> (bool, bool) {
    let has_a = s.contains('a');
    let has_z = s.contains('z');
    (has_a, has_z)
}

fn main() {
    println!("{}", apply_to_jobs(42, "Software Engineer"));
    println!("Is 42 even? {}", is_even(42));
    println!("Is 7 even? {}", is_even(7));
    println!("{:?}", alphabets("aardvark")); -> (true, false)
    println!("{:?}", alphabets("zoology"));  -> (false, true)
    println!("{:?}", alphabets("zebra"));    -> (true, true)
}
