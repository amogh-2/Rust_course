fn process_numbers(nums : &[i32]){
    let mut sum = 0;
    for num in nums{
        sum+=num;
    }
    println!("The sum of the numbers is {}.", sum);
}

fn main(){
    let nums = [4,2,3,6];
    process_numbers(&nums);
}