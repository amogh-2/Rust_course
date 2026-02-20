enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64)
}

fn main(){
    let shape = vec![Shape::Circle(5.0),Shape::Square(4.0),Shape::Rectangle(3.0,6.0)];
    let total_area: f64 = shape
        .iter()
        .map(|shape| match shape{
            Shape::Circle(radius) => std::f64::consts::PI*radius*radius,
            Shape::Square(length) => length*length,
            Shape::Rectangle(length,breadth) => length*breadth, 
        })
        .sum();

    dbg!("The total area is: {:.2} sq. units ", total_area);
}