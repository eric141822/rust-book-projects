use std::ops::Deref;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn cons() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    // loop through the list.
    let mut curr = &list;
    while let List::Cons(val, next) = curr {
        println!("val: {}", val);
        curr = next;
    }
}

// * operator can also deref Box<T> to T.
fn box_works_like_pointer() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // behind the scenes, Rust calls *(y.deref())
    assert_eq!(5, *y);

    println!("It works!");
}

// our own box
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // defines an associated type for the Deref trait to use.
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 //.0 access the first element of the tuple.
    }
}

fn my_box_works_like_pointer() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // behind the scenes, Rust calls *(y.deref())
    assert_eq!(5, *y);

    println!("MyBox works!");
}

fn main() {
    cons();
    box_works_like_pointer();
    my_box_works_like_pointer();
}
