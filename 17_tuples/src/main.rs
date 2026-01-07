fn main() {
    let employee: (&str, i32, f64) = ("Alice", 30, 50000.0);
    // Print the entire tuple using Debug trait
    println!("Employee details: {:?}", employee);
    // Pretty print
    println!("Employee details (pretty): {:#?}", employee);
    // Accessing tuple elements by index
    println!("Name: {}", employee.0);
    println!("Age: {}", employee.1);
    println!("Salary: {}", employee.2);

    // Destructuring the tuple
    let (name, age, salary) = employee;
    println!("Destructured - Name: {}, Age: {}, Salary: {}", name, age, salary);

    // Modifying a tuple (by creating a new one)
    let mut modified_employee = employee;
    modified_employee.1 += 1; // Increment age
    modified_employee.2 += 5000.0; // Increment salary
    println!("Modified Employee details: {:?}", modified_employee); 

    // Tuple length (using std::mem::size_of_val to get size in bytes)
    println!("Size of the tuple in bytes: {}", std::mem::size_of_val(&employee));

    // Nested tuples
    let nested_tuple: (&str, (i32, f64)) = ("Bob", (25, 60000.0));
    println!("Nested tuple: {:?}", nested_tuple);   

    // Accessing elements in nested tuples
    println!("Name: {}", nested_tuple.0);
    println!("Age: {}", (nested_tuple.1).0);
    println!("Salary: {}", (nested_tuple.1).1);
}
