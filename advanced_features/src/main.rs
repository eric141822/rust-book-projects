use std::slice;

use advanced_features::*;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
// The function below will not compile because it is trying to return two mutable references to the same variable.

/* 
fn split_at_mut_broken(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    
    assert!(mid <= len);
    
    (&mut values[..mid], &mut values[mid..])
}
*/

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    
    assert!(mid <= len);
    
    // Using raw pointers to bypass the restriction of returning two mutable references to the same variable.
    let ptr = values.as_mut_ptr();
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

// Using extern to call the C function abs.
// Calling external functions is considered unsafe because the compiler cannot verify that the function is correct.
extern "C" {
    fn abs(input: i32) -> i32;
}

// Static mutable global variables are unsafe because they can be accessed from multiple threads.
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Dynamically sized type and the Sized trait
// ?Sized is a trait bound that allows the function to accept both references to sized and unsized types.
// this syntax is only allowed for the Sized trait.

/*
fn generic<T: ?Sized>(t: &T) {
    // code here
}
*/
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32; // unsafe raw pointer
    let r2 = &mut num as *mut i32; // unsafe mutable raw pointer

    // unsafe block, you can only dereference raw pointers in unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Usually, rust does not allow immutable and mutable references to the same variable.
    // But, raw pointers can be used to bypass this restriction.
    // This is why raw pointers are unsafe.

    let mut v = [1, 2, 3, 4, 5, 6];

    let (left, right) = split_at_mut(&mut v, 3);
    
    assert_eq!(left, &mut [1, 2, 3]);
    assert_eq!(right, &mut [4, 5, 6]);

    println!("left: {:?}", left);
    println!("right: {:?}", right);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_counter(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog (animal) is called a {}", <Dog as Animal>::baby_name());

    let p: Point = Point { x: 1, y: 0 };
    p.outline_print();

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = my_vec![1, 2, 3];
    println!("List of numbers: {:?}", list_of_numbers);

    Pancakes::hello_macro();
}
