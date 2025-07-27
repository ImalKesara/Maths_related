use std::io;
fn main() {
    println!("fahrenheit to celsius");
    println!("enter your value");
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("failed to readline");

    let fahrenheit: i32 = fahrenheit.trim().parse().expect("please enter a number");

    fn calculate_c(&fahrenheit: i32) -> i32 {
        let celsius: i32 = (fahrenheit - 32) * 5 / 9;
    }

    println!("{calculate_c(fahrenheit)}°C");
}
