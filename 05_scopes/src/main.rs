fn main() {
    let outer_scope_variable: i32 = 42;
    {
        let inner_scope_variable: i32 = 7;
        println!("Inner scope variable: {}", inner_scope_variable);
        println!("Accessing outer scope variable from inner scope: {}", outer_scope_variable);
    }
    // The following line would cause a compile-time error because inner_scope_variable is not accessible here
    // println!("Trying to access inner scope variable from outer scope: {}", inner_scope_variable);
    println!("Outer scope variable: {}", outer_scope_variable);
}
