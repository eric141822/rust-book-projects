pub fn add_one(x: i32) -> i32 {
    x + 1
}

// This function takes a function as an argument and calls it twice.
pub fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// does not compile, size of Fn(i32) -> i32 is not known at compile time
/*
pub fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
*/

// This function returns a closure.
pub fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}