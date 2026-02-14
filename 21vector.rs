fn get_item(){
    let index = 3;
    let vec = vec![1,2,3,4,5];
    let value = vec.get(index).unwrap();
    println!("The value at index {} is {:?}",index,value);
}

fn main(){
    get_item();
    let vec = vec![1,2,3,4,5];
    let value= vec[2];
    println!("The 3rd value is: {}",value);

    let last_value=vec.last().unwrap();
    println!("The last value is: {}", last_value);
    
    match vec.first(){
        Some(first_value) => println!("The first value is: {}", first_value),
        None =>println!("The vector is empty"),
    }
}