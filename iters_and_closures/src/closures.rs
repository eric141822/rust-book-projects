use std::{vec, thread};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
        }
    }

    pub fn giveaway(&self, color: Option<ShirtColor>) -> ShirtColor {
        // closure that returns the most stocked color if the user has no preference
        color.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut counts = (0, 0);
        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => counts.0 += 1,
                ShirtColor::Blue => counts.1 += 1,
            }
        }
        if counts.0 > counts.1 {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn closure_borrow() {
    let list = vec![1, 2, 3, 4, 5];
    println!("before defining closure, list = {:?}", list);

    let only_borrows = || println!("Within closure, list = {:?}", list);

    println!("before calling closure, list = {:?}", list);
    only_borrows();
    println!("after calling closure, list = {:?}", list);
}

pub fn closure_mutable() {
    let mut list = vec![1, 2, 3, 4, 5];
    println!("before defining closure, list = {:?}", list);
    let mut borrows_mutably = || list.push(7);
    
    // this line will fail to compile if uncommented
    // list is borrowed mutably in the closure, so it cannot be borrowed immutably here
    // println!("before calling closure, list = {:?}", list);
    borrows_mutably();
    println!("after calling closure, list = {:?}", list);
}

pub fn closure_thread_move() {
    let list = vec![1,2,3,4,5];
    println!("before defining closure, list = {:?}", list);

    // reference in thread normally is immutable, ownership is not moved.
    // with `move`, ownership is moved to the thread.
    // `move` is required to use variables from outside the thread.
    thread::spawn(move || {
        println!("From thread, list = {:?}", list);
    }).join().unwrap();
}
