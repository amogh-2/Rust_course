fn main(){
    for i in 1..=11{
        if i%2 == 0{
            continue;
        }
        else if i ==9{
            break;
        }
        else {
            println!("The number {} is odd",i);  
        }
    }
}