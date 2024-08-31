struct Cicle {
    pi: f64,
    radius: f64
}

fn main() {
    let circle1: Cicle = Cicle {
       pi: 3.14,
       radius: 3.0
    };
    println!("Area of the circle is : {}", area(&circle1));
}

fn area(circle: &Cicle) -> f64 {
    circle.pi * circle.radius * circle.radius
}
