//use std::io;

fn main(){
    let maybe_number = Some(Some(32));
    if let Some(Some(number)) = maybe_number {
        println!("The number is {}", number);
    }
    else {
        println!("There is no number");
    }
}