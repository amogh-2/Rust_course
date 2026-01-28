use std::io;
fn main(){
    println!("Enter a number: ");
    let mut x= String::new();
    io::stdin().read_line(&mut x).expect("Enter a valid number");
    let x= x.trim().parse::<i32>().expect("Enter a valid number");
    let x= x+5;
    let token = {
        if x<=9{
            "Single digit number"
        }
        else{
            "Two digit number"
        }
    };
    println!("{}", token);
}
