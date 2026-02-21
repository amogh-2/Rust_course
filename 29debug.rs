// I tried the codeLLDB debugger on here hence the name

use std::io;

fn main(){
    let mut word = String::new();
    while word.to_lowercase().trim()!= "stop"{
        word.clear();
        println!("Please Enter a word (type 'stop' to exit)");
        io::stdin().read_line(&mut word).unwrap();
        println!("You entered {}", word);
    }
    println!("Goodbye!");
}