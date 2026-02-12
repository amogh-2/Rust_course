#[derive(Debug)]

struct Person{
    f_name:String,
    _l_name:String,
    _age:Option<i32>,
}

fn main(){
    let amogh = Person{
        f_name: "Amogh".to_string(), _l_name: "Bhattarai".to_string(), _age:None,//_age: None ko satta i32 rakhna maan vaye Some(i32) garna paryo 
    };
    println!("The first name is: {}",amogh.f_name);
}
