use std::io;

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    println!("Enter the numbers to sum: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error taking input");
    
    for num_str in input.split_whitespace() {
        let number = num_str.trim().parse::<i32>().expect("Enter a valid number");
        numbers.push(number);
    }
    
    let result = sum(&numbers);
    println!("The sum is {}", result);
}
