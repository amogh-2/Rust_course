// fn split_string(s: String, symbol: char, field:usize )-> String{
// }
// fn main(){
//     let chunk = split_string("hello".to_string(),',',1);
//     println!("Split String: {}", chunk);
// }

fn merge(words: &[String])-> String{
    let mut sentence= "".to_string();
    for word in words{
        sentence+=word;
    }
    sentence
}

fn main(){
    let words = ["Hello ".to_string(), "World".to_string(), "!".to_string()];
    let result = merge(&words);
    println!("{}",result);
}