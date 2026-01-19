fn main() {
    println!("Fibonacci sequence of 4: {:?}", fibonacci(4));
    println!("Fibonacci sequenceof 10: {:?}", fibonacci(10));
}

// Returns a vector containing the Fibonacci sequence up to the n-th number.
fn fibonacci(n: u32) -> Vec<u32> {
    if n == 0 {
        vec![0]
    } else if n == 1 {
        vec![1]
    } else if n == 2 {
        vec![1, 1]
    } else {
        let mut seq = fibonacci(n - 1);
        let next_value = seq[(n - 2) as usize] + seq[(n - 3) as usize];
        seq.push(next_value);
        seq
    }
}