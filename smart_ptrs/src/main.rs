use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
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

// Deref coercion
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

/*
    Deref coercion with Mutability -> DerefMut
    Rust does deref coercion when it finds types and trait implementations in three cases:
    1. From &T to &U when T: Deref<Target=U>
    2. From &mut T to &mut U when T: DerefMut<Target=U>
    3. From &mut T to &U when T: Deref<Target=U>
*/

struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(data: &str) -> CustomSmartPointer {
        CustomSmartPointer {
            data: data.to_string(),
        }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Rc<T> enables multiple ownership.
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}
fn rc_demo() {
    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcList::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// RefCell<T> and the Interior Mutability Pattern
// RefCell<T> is a smart pointer type that allows mutable borrows checked at runtime.
// RefCell<T> is only for single-threaded scenarios.

// Interior mutability is a design pattern in Rust that allows you
// to mutate data even when there are immutable references to that data.
// To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules

// Box<T> vs Rc<T> vs RefCell<T>
// Box<T> is for single ownership. For both immutable and mutable data.
// Rc<T> is for multiple ownership. Only for immutable data.
// RefCell<T> is for single ownership with mutable borrows checked at runtime. For immutable and mutable data.

// Allowing multiple owners of mutable data by combining Rc<T> and RefCell<T>
#[derive(Debug)]
enum ListV3 {
    Cons(Rc<RefCell<i32>>, Rc<ListV3>),
    Nil,
}

fn list_v3_demo() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ListV3::Cons(Rc::clone(&value), Rc::new(ListV3::Nil)));
    let b = ListV3::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = ListV3::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Memory Leak Example
#[derive(Debug)]
enum ListV4 {
    Cons(i32, RefCell<Rc<ListV4>>),
    Nil,
}

impl ListV4 {
    fn tail(&self) -> Option<&RefCell<Rc<ListV4>>> {
        match self {
            ListV4::Cons(_, item) => Some(item),
            ListV4::Nil => None,
        }
    }
}

fn memory_leak_demo() {
    let a = Rc::new(ListV4::Cons(5, RefCell::new(Rc::new(ListV4::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(ListV4::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

fn main() {
    cons();
    box_works_like_pointer();
    my_box_works_like_pointer();

    // deref coercion
    let m = MyBox::new(String::from("Rust"));
    // &MyBox<String> -> &String -> &str
    hello(&m);
    // if no deref coercion, we have to write like this.
    // hello(&(*m)[..]);

    // Dropping
    let c = CustomSmartPointer::new("my stuff");
    let d = CustomSmartPointer::new("other stuff");
    println!("CustomSmartPointers created.");

    // explicit drop
    let c = CustomSmartPointer::new("explicit drop");
    // cannot call c.drop() to prevent double free.
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // Rc<T>
    rc_demo();

    // RefCell<T> + Rc<T>
    list_v3_demo();

    // Memory Leak Example
    memory_leak_demo();

    // Weak references
    smart_ptrs::tree_demo();
}
