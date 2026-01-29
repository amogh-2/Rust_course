use std::io;

fn main(){
    let mut x = String::new();
    while x.trim() != "stop"{
        x.clear();
        println!("Please enter a word (Enter 'stop' to exit): ");
        io::stdin().read_line(&mut x).unwrap();
        println!("\nYour word is {}.", x);

    } 
}