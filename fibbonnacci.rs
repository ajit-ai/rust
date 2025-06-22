// ...existing code...

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("Hello, OneCompiler!");

    // ...existing code...

    // Example: Find the 10th Fibonacci number
    let n = 10;
    println!("Fibonacci({}) = {}", n, fibonacci(n));
}