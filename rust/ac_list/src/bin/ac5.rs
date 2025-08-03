use std::io;
fn main() {
    loop {
        println!("Enter your name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("failed to read");

        let name = name.trim();

        match name {
            "namal" => {
                println!("Welcome {name}");
                break;
            }
            "bobb" => println!("wrong user"),
            "quit" => break,
            _ => println!("try again"),
        }
    }
}
