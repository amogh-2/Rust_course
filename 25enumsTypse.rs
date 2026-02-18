#[derive(Debug)]

enum Red{
    Crimson,
    Burgundy,
    Maroon,
}

struct Color{
    name: String,
    shade: Red,
}

fn supported_color(r: Red){
    match r{
        Red::Maroon => println!("Picked the correct shade: {:?}!",r),
        _=> println!("Incorrect shade picked."),
    }
}

fn main(){
    let color1 = Color{
        name: String::from("Red"),
        shade: Red::Crimson,
    };

    let color2 = Color{
        name: String::from("Red"),
        shade: Red::Maroon,
    };

    println!("Color 1: {} of shade {:?}", color1.name, color1.shade);
    println!("Color 2: {} of shade {:?}", color2.name, color2.shade);
    supported_color(color1.shade);
    supported_color(color2.shade);
    
}
