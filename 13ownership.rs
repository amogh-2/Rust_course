
fn printint(number: i32){
    println!("{}",number);
}


fn printstr(word:&String ){
    println!("{}",word);
}

fn printvec(vec: &Vec<i32>){
    println!("{:?}",vec);
}
fn main(){
    let word = "HELLO".to_string();
    printstr(&word);
    let num = 2;
    printint(num);
    let a_vec = vec![1,2,3,4,5];
    printvec(&a_vec);
}