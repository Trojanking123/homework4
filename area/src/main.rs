#[derive(Debug, Clone, Copy)]
struct Circle {
    radius : f32
}
#[derive(Debug, Clone, Copy)]
struct Rectangle {
    high : f32,
    width: f32,
}


trait  Area {
    fn area(self) -> f32;
}

impl Area for Circle {
    fn area(self) -> f32 {
        return 3.14 * self.radius * self.radius;
    }
}

impl Area for Rectangle {
    fn area(self) -> f32 {
        return self.high * self.width;
    }
    
}

fn get_area<T: Area>(x: T) -> f32 {
    return x.area();
}

fn main() {
    let circle = Circle {
        radius: 3.0,
    };

    let rec = Rectangle {
        high: 4.0,
        width: 8.0,
    };

    println!("area of {:?} is: {:?}", circle, get_area(circle));
    println!("area of {:?} is: {:?}", rec, get_area(rec));
}
