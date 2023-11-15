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
