fn main() {
    let mut countdown = 25;
    loop {
            // In case of using a loop, always ensure there's a way to break out of it!
            if countdown <= 0 {
                println!("Liftoff!");
                break;
            }
            
            // If there is a condition where you want to skip the rest of the loop 
            // and start the next iteration, use 'continue'.
            if countdown % 2 == 0 {
                println!("Abnormal error fast-tracking countdown by -3");
                countdown -= 3;
                println!("T-minus {} seconds", countdown);
                continue;
                // Notice that the code below 'continue' is skipped when the condition is met.
                // countdown is not reduced further by 1 in this case.
            }
            countdown -= 1;
            println!("T-minus {} seconds", countdown);
            
    }
}
