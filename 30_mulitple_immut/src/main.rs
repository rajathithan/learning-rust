fn main() {
    // Multiple immutable references to the same data
    // ownership rules allow multiple immutable references to the same data, 
    // but only one mutable reference at a time. 
    // This ensures that data cannot be modified while it is being read, 
    // preventing data races.
    let car_color = String::from("red");
    let ref1 = &car_color;
    let ref2 = &car_color;
    println!("The color of the car is: {}", ref1);
    println!("The color of the car is: {}", ref2);

    // Mutable reference to the same data with restrictions
    // if we try creating 2 mutable references to the same data, 
    // it will cause a compile-time error because it violates the ownership rules.

    let mut car_motor = String::from("V8");
    let ref3: &mut String = &mut car_motor;
    println!("The motor of the car is: {}", ref3);
    
    // Immutatable reference to the same data with has a mutable reference is not allowed

    //let ref4: &String = &car_motor;
    //println!("The motor of the car is: {}", ref4);

    // Second Mutable reference to the same data is also not allowed

    //let ref5: &mut String = &mut car_motor;
    //println!("The motor of the car is: {}", ref5);


}
