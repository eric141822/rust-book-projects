use std::fmt::Display;
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime annotations in struct definitions
// An instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Combine lifetime annotations with generic type parameters and trait bounds.
// Only executes when the generic type T implements the Display trait
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);

    println!("---------------------");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("First sentence: {}", i.part);

    println!("---------------------");

    let s1 = String::from("long string is long");
    let s2 = String::from("xyz");
    let ann = String::from("This is an announcement!");
    let result = longest_with_announcement(s1.as_str(), s2.as_str(), ann.as_str());
    println!("The longest string is {}", result);
}
