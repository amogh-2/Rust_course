use std::io;

fn main(){
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    println!("If you've just joined enter 'new' else enter 'continue'");
    let mut greeting = String::new();
    io::stdin().read_line(&mut greeting).unwrap();
    let greeting = greeting.trim();
    match greeting{
        "new"=>{
            println!("Hello {} the unknown traveller.",name);
        },
        "continue"=>{
            println!("Welcome back my dear friend {}.", name);
        },
        _=>{
            println!("You should have entered a valid input {}, you're dumb.", name);
        }
    }    
}