use std::io;
fn display_something(value:u32){
    if value > 5 {
        println!(">5");
    }else if value < 5 {
        println!("<5");
    }else{
        println!("=5");
    }
}

fn main(){
    println!("Control flow 2");
    println!("Enter number");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("failed to read");
    let value : u32 = value.trim().parse().expect("please type a number");
    display_something(value);
}
