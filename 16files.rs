use std::fs;
use std::env;

fn path_spec()-> String{
    let x: Vec<String> = env::args().collect();
    // println!("The path is: {}.", x[1]);
    x[1].clone()
}

fn main(){
    let result = fs::read_to_string(path_spec());
    let a_file = match result {
        Ok(content) => content,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("The file doesn't exist");
                }
                _ => {
                    panic!("There is an error finding the file: {}", error);
                }
            }
        }
    };
    println!("{}", a_file);
}