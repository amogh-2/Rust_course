use std::io;

fn vec_input() -> i32 {
    let mut a_vec: Vec<i32> = Vec::new();
    
    loop {
        println!("Enter a number (or 'q' to quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error taking input");
        let input = input.trim();
        
        if input == "q" || input == "Q" {
            break;
        }
        
        if let Ok(number) = input.parse::<i32>() {
            a_vec.push(number);
            println!("Number added. Sum so far: {}", a_vec.iter().sum::<i32>());
        } else {
            println!("Enter a valid number");
        }
    }
    
    a_vec.into_iter().sum()
}

fn main() {
    println!("Sum: {}", vec_input());
}
