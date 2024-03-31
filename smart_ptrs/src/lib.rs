use std::cell::RefCell;
use std::rc::{Rc, Weak};
pub trait Messenger {
    fn send(&self, msg: &str);
}
// 'a lifetime -> ensures the instance of LimitTracker does not outlive the reference to Messenger.
// T: Messenger -> trait bound syntax. It means the type T must implement the Messenger trait.
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[derive(Debug)]
pub struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        })
    }
}

pub fn tree_demo() {
    let leaf = Node::new(3);
    let branch = Node::new(5);

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    // leaf has a parent.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // branch has a child.
    branch.children.borrow_mut().push(leaf.clone());
    println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // with just Vec<String> we can't borrow the vector mutably and immutably at the same time.
    // since send() references the vector immutably, we can't borrow it mutably in the test.
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }

        /*
        this implementation will panic due to a runtime error.
        Tried to borrow the RefCell<Vec<String>> mutably twice.
        */
        // fn send(&self, message: &str) {
        //     let mut one_borrow = self.sent_messages.borrow_mut();
        //     let mut two_borrow = self.sent_messages.borrow_mut();

        //     one_borrow.push(String::from(message));
        //     two_borrow.push(String::from(message));
        // }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
