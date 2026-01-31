fn loop_and_panic(numbers: i32){
    for i in (-1..numbers).rev(){
        if i<0{
            panic!("Donot enter negative numbers");
        }
        println!("{}",i);
    }
}

fn main(){
    let numbers=5;
    loop_and_panic(numbers);
}