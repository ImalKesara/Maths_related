use std::io;
fn first_name() -> String {
    let mut name = String::new(); 
    println!("enter first name");
    io::stdin().read_line(&mut name).expect("Failed to read");
    name
}
fn last_name() -> String {
    let mut lname = String::new();
    println!("enter last name");
    io::stdin().read_line(&mut lname).expect("failed to read");
    lname
}

fn main() {
    let fname = first_name();
    let lname = last_name();
    println!("you're name is {fname} - {lname}");
}
