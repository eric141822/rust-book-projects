use std::{io, collections::HashMap};
fn main() {
    let mut d: HashMap<String, Vec<String>> = HashMap::new();
    println!("Enter 5 employees in the format Add <name> to <department>:");
    for _  in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let split_input: Vec<&str> = input.split_whitespace().collect();
        d.entry(split_input[3].to_string()).or_insert(Vec::new()).push(split_input[1].to_string());
    }
    let mut departments: Vec<String> = d.keys().cloned().collect();
    departments.sort();
    let n = departments.len();
    loop {
        println!("Choose a department:");
        let mut i = 0;
        for (_, department) in departments.iter().enumerate() {
            println!("{}. {}", i, department);
            i+=1;
        }
        println!("{}. {}", i, "Get all employees by department");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: usize = choice.trim().parse().expect("Please type a number!");
        if choice < n {
            let mut employees = d.get_mut(&departments[choice]).unwrap().clone();
            employees.sort();
            println!("Employees in {}:, {:?}", departments[choice], employees);
        } else if choice == n {
            for dept in &departments {
                let mut employees = d.get_mut(dept).unwrap().clone();
                employees.sort();
                println!("Employees in {}:, {:?}", dept, employees);
            }
        } else {
            println!("Invalid choice");
        }
    }
}
