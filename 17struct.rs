#[derive(Debug)]

struct Person{
    _f_name:String,
    _l_name:String,
    _age:Option<i32>,
}

fn main(){
    let amogh = Person{
        _f_name: "Amogh".to_string(), _l_name: "Bhattarai".to_string(), _age:None, 
    };
    println!("The first name is: {}",amogh._f_name);
}
