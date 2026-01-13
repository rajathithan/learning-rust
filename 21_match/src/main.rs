fn main() {
    let season: &str = "autumn";
    let prime_number: i32 = 7;
    let number: i32 = 15;

    // Using match to check multiple conditions
    match season {
        // Match arms with curly braces
        "spring" => {
            println!("It's spring!");
        }
        "summer" =>{
            println!("It's summer!");
        }
        "autumn" => {
            println!("It's autumn!");
        }
        "winter" => {
            println!("It's winter!");
        }
        _ => {
            println!("Unknown season.");
        },
    }

    match prime_number {
        // Multiple patterns in a single arm
        // single line without curly braces
        2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => println!("{} is a prime number less than 20.", prime_number),
        _ => println!("{} is not a prime number less than 20.", prime_number),
    }

    match number{
        // Match guards with if conditions
        n if n % 2 == 0 => println!("The number is even"),
        n if n % 2 != 0 => println!("The number is odd"),
        // unreachable arm, this is just to satisfy exhaustiveness
        _ => unreachable!(),
    }

}
