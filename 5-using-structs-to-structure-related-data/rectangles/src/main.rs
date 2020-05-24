#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1: {:?}", rect1);
    println!("rect1: {:#?}", rect1);
    println!(
        "The area of the rectangle is {:?} square pixels.",
        area(&rect1)
    );

    println!(
        "using method instead the area 2is {:?} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sqr1 = Rectangle::square(3);
    println!("sqr1 {:#?}", sqr1)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
