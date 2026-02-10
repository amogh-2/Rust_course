use std::fs;
use std::env;

fn path_spec()-> String{
    let x: Vec<String> = env::args().collect();
    // println!("The path is: {}.", x[1]);
    x[1].clone()
}

fn main(){
    let a_file = fs::read_to_string(path_spec()).expect("Failed to read");
    println!("{}",a_file);
     
}