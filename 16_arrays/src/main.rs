fn main() {
    // Declare an array of integers
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    // The below println would cause a compile-time error 
    // because arrays do not implement Display trait
    // println("Array of numbers: {}", numbers);
    // This is a format mandated by the Debug trait
    println!("Array of numbers {:?}", numbers);
    // pretty print
    println!("Array of numbers {:#?}", numbers);

    // Another way to print is to use the Debug Macro
    dbg!(&numbers);

    // Accessing elements by index
    println!("First element: {}", numbers[0]);
    println!("Third element: {}", numbers[2]);

    // Iterating over the array
    for (index, &value) in numbers.iter().enumerate() {
        println!("Element at index {}: {}", index, value);  
    }

    // Modify an array (using a mutable array)
    let mut mutable_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    mutable_numbers[0] = 10;
    println!("Modified array: {:?}", mutable_numbers);

    // Array length
    println!("Length of the array: {}", numbers.len()); 

    // Slicing an array
    let slice: &[i32] = &numbers[1..4];
    println!("Slice of the array (indices 1 to 3): {:?}", slice);

    // Multidimensional array
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("2D Array (Matrix): {:?}", matrix);

    // Accessing elements in a multidimensional array
    println!("Element at (1,2): {}", matrix[1][2]);

    // Iterating over a multidimensional array using pointer references
    for row in &matrix {
        for &value in row {
            print!("{} ", value);
        }
        println!(); 
    }

    // Iterating over a multidimensional array using indices
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            println!("Element at ({}, {}): {}", i, j, matrix[i][j]);
        }
    }

    // Using array in functions
    fn sum_array(arr: &[i32]) -> i32 {
        arr.iter().sum()    
    }
    let total = sum_array(&numbers);
    println!("Sum of array elements: {}", total);

    // Copying arrays
    let copied_array = numbers;
    println!("Copied array: {:?}", copied_array);


}
