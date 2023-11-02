// function to get nth fibonacci number.

fn main() {
    println!("Enter n for get nth fibonacci: ");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Please type a number!");
    println!("Fibonacci number is: {}", fib_iterative(n));
    println!("Fibonacci number (in recursion) is: {}", fib_recursion(n));
}

fn fib_recursion(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib_recursion(n - 1) + fib_recursion(n - 2)
}

fn fib_iterative(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }
    c
}