use std::fs::File;
use std::io;
// fn read_username_from_file() -> Result<String, io::Error> {
//     let user_file_result = File::open("hello.txt");
//     let mut file = match user_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();
//     match file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file2() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut String::new())?;
//     Ok(username)
// }

fn read_username_from_file3() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

fn main() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    let username = read_username_from_file3().expect("Failed to read username from file");

    println!("Hello, {}!", username);
}



