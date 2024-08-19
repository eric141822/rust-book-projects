# Rust Book Practices

A collection of practice done according to The Rust Programming Language book by Steve Klabnik and Carol Nichols.

## Chapter 1 Introduction

### [Hello Cargo](hello_cargo/)

An introduction to cargo. Simply prints "Hello, World."

## Chapter 2 Guessing Game

### [Guessing Game](guessing_game/)

Guess a secret number ranging from 1 to 100.

## Chapter 3 Common Programming Patterns

### [Farenheit / Celcius Converter](ftoc/)

Convert from Farenheit to Celcius, or vice versa.

### [Fibonacci](fibonacci/)

Calculates the N-th fibonacci number in both a recursive way or a iterative way.

### [Christmas Carol](christmas_carol/)

Prints the lyrics to [The Twelve Days of Christmas](https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics).

## Chapter 5, 6 Structs and Enums

### [Rectangle](rectangle/)

An introduction on structs (with Rectangle) in Rust. `fmt::Display` for Rectangle is implemented instead of using the `#[derive(Debug)]` trait and `dbg!()`.

## Chapter 7 Packages, Crates, Modules

### [Restaurant](restaurant/)

From Chapter 7, a module to emulate a restaurant. An introduction to syntax and structures related to rust modules, such as paths, `pub`, `use`, etc.

## Chapter 8 Common Collections

### [Median and Mode from Vector](vec_median_mode/)

Gets the median and the mode from a vector of integers.

### [Pig Latin](pig_latin/)

Converts a string input into [Pig Latin](https://en.wikipedia.org/wiki/Pig_Latin).

### [Employees in a Company](employee_names/)

Input employees by department in the format _Add \<Employee\> to \<Department\>_.

Lets user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

## Chapter 9 Error Handling

### [Common Error Handling Patterns](error_handling/)

A collection of common error handling patterns in Rust.

### [Guessing Game with Error Handling](guessing_game/)

Guess a secret number ranging from 1 to 100. Now with error handling!

## Chapter 10 Generic Types, Traits and Lifetimes

### [Generics](generics/)

Generic types in Rust.

### [Traits](aggregator/)

Traits in Rust. Exploring concepts such as `impl`, `trait bounds`, `default implementations`, etc.

### [Lifetimes](lifetime/)

Lifetime concepts in Rust. Exploring concepts such as `lifetime annotations`, `lifetime elision`, etc.

## Chapter 11 Writing Automated Tests

### [Testing in Rust](automated_tests/)

Testing in Rust. Exploring concepts such as `#[cfg(test)]`, `#[test]`, `#[should_panic]`, `#[should_panic(expected="")]`, `#[ignore]`, the `test` directory etc.
To run tests, use `cargo test`.
To run tests consecutively and not in parallel (default) use `cargo test -- --test-threads=1`.
To run tests with function outputs, use `cargo test -- --show-output`.
To run only the ignored tests, use `cargo test -- --ignored`.
To test specific functions, use `cargo test <function_name>`.
To run specific test crates in the `tests` directory, use `cargo test --test <test_crate_name>`.
Private functions can be tested in Rust.

## Chapter 12 An I/O Project: Building a Command Line Program

A simple grep clone written in Rust. See [minigrep](https://github.com/eric141822/minigrep) for details.

## Chapter 13 Functional Language Features: Iterators and Closures

### [Iterators and Closures](iters_and_closures/)

Exploring concepts such as `iter`, `into_iter`, `map`, `filter`, `collect`, closures in general.

## Chapter 14 More about Cargo and Crates.io

### [Crate IO](crate_io/)

Exploring concepts related to Crate IO and how to publish crates, cmds such as `cargo publish`, `cargo yank`... etc.

### [Workspaces](add/)

Workspaces in Rust, `member`, `package`, `workspace`, `dependencies`... etc.

## Chapter 15 Smart Pointers

### [Smart Pointers](smart_ptrs/)

Smart pointers in Rust.

1. `Box<T>`, `Deref` trait, `Drop` trait
2. `Rc<T>`, `RefCell<T>`, `Weak<T>`.
3. Demos on `Rc<T>` and `RefCell<T>`.
4. Possible Memory Leaks with `Rc<T>` and `RefCell<T>`.
5. Tree data structure with `Rc<T>` and `RefCell<T>` to demonstrate usage for Weak references.

## Chapter 16 Fearless Concurrency

### [Concurrency](concurrency/)

Introduction to concurrency in Rust. Basic concepts such as `thread`, `spawn`, `join`, `move`, `Mutex`, `Arc`, `Send`, `Sync`, `channels`, `mpsc`, etc.
Mutexes in Rust while similar to other languages, are more strict in Rust due to the borrow checker, usually paird with `Arc` (Atomic Reference Count) to share data between threads.

## Chapter 17 Object Oriented Programming

### [Object Oriented Programming](oop/)

Basics of Object Oriented Programming in Rust.

### [Blog](blog/)

A simple blog implementation in Rust. Shows how to implement OOP concepts in Rust such as using Traits to emulate inheritance, and using `Box<dyn Trait>` to emulate polymorphism.
