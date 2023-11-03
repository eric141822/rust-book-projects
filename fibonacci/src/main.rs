// function to get nth fibonacci number.

fn main() {
    println!("Enter n for get nth fibonacci: ");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u64 = n.trim().parse().expect("Please type a number!");
    println!("Fibonacci number is: {}", fib_iterative(n));
    if n <= 30 {
        println!("Fibonacci number (in recursion) is: {}", fib_recursion(n));
    } else {
        println!("Recursion is too slow for large n = {n}");
    }
}

fn fib_recursion(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib_recursion(n - 1) + fib_recursion(n - 2)
}

fn fib_iterative(n: u64) -> u64 {
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