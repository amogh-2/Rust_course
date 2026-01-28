use std::io;

fn main(){
    println!("Is Rust a systems programming language? (true/false)");
    let mut decision=String::new();
    io::stdin().read_line(&mut decision).unwrap();
    let decision:bool=decision.trim().parse().unwrap();
    if decision{
        println!("Correct");
    }
    else{
        println!("Incorrect");
    }
}