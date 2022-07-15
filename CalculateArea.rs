fn main() {
    let triangle = Triangle {
        a: 10f32,
        h: 10f32
    };
    println!("The area of triangle is {:?}", calculate_area(&triangle));
    let circle = Circle {
        r: 10f32
    };
    println!("The area of circle is {:?}", calculate_area(&circle));
    let rectangle = Rectangle {
        a: 10f32,
        b: 10f32
    };
    println!("The area of rectangle is {:?}", calculate_area(&rectangle));
}

pub trait Area {
    fn calculate(&self) -> f32;
}

struct Triangle {
    a: f32,
    h: f32,
}
struct Circle {
    r: f32
}
struct Rectangle {
    a: f32,
    b: f32,
}

impl Area for Triangle {
    fn calculate(&self) -> f32 {
        return self.a * self.h / 2f32;
    }
}

impl Area for Circle {
    fn calculate(&self) -> f32 {
        return self.r.powf(2.0) * std::f32::consts::PI;
    }
}

impl Area for Rectangle {
    fn calculate(&self) -> f32 {
        return self.a * self.b;
    }
}

pub fn calculate_area<T: Area>(shape: &T) -> f32{
    return shape.calculate();
}
