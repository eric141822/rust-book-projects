struct Point {
    x: i32,
    y: i32,
}

// Rgb is unused so we get a warning
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

// Quit, Move, Write are unused but we don't get a warning because they start with _.
enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    _Write(String),
    ChangeColor(Color),
}

enum HelloMessage {
    Hello { id: i32 },
}

fn main() {
    let x = Some(5);
    let y = 10;

    // second arm of the match statement introduces a new variable y that is only valid within that arm
    // the y in the outer scope is not shadowed by the y in the arm
    // the y in the arm is a new variable that contains the value in the Some variant
    // so the second arm would match and y binds to the value 5.
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("At the end: x = {:?}, y = {:?}", x, y);

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::_Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::_Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::_Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, ..,  fifth) => {
            println!("first and last number: {}, {}", first, fifth);
        }
    }

    // match guard: additional if condition after the pattern

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // Similar to the first case, but using match guard to avoid shadowing
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, y = {:?}", n), // n == 5 != 10, so this arm is skipped
        _ => println!("Default case, x = {:?}", x),
    }

    let msg = HelloMessage::Hello { id: 5 };
    match msg {
        HelloMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        HelloMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        HelloMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}
