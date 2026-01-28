use std::io;

fn main(){
   let mut x= String::new();
    loop{
        println!("Enter a number: ");
        io::stdin().read_line(&mut x).expect("An error occured");
        match x.trim().parse::<i32>(){
            Ok(_)=>{
                break;
            }
            Err(_)=>{
                println!("Enter a valid number");
            }
        }
    }
    let mut x= x.trim().parse::<i32>().unwrap();
    loop{
        println!("The value of x is {}", x);
        x+=1;
        if x>5{
            break;
        }
    }
}