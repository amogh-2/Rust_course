use std::io;

fn main(){
    let digit : f32 = 6.0;
    println!("Enter an alphabet:");
    let mut alphabet = String::new();
    io::stdin().read_line(&mut alphabet).unwrap();
    println!("The value of {} is {}", alphabet, digit);
}