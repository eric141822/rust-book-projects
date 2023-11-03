struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl std::fmt::Display for Rectangle {
    // <'_> lets Rust infer the lifetime of the reference
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::square(20);

    println!("rect1 is {}", rect1);
    println!("rect2 is {}", rect2);

    println!(
        "The area of the rect1 is {} square pixels.\nThe area of the rect2 is {} square pixels.",
        rect1.area(), rect2.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
