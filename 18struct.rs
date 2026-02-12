struct User{
    name: String,
    age:u32,
    active: bool,
}

impl User{
    fn new_user(name:String, age: u32)-> Self{
        Self{
            name,
            age,
            active: true,
        }
    }
    fn deactivate(&mut self){
        self.active= false;
    }
}

fn main(){
    let mut a_user=User::new_user(
        String::from("Amogh"),
        32,
    );
    println!("Account {} aged {}'s active status is: {}",a_user.name,a_user.age,a_user.active);
    a_user.deactivate();
    println!("Account {} aged {}'s active status is: {}", a_user.name,a_user.age, a_user.active);
}