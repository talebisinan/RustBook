struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 10, height: 10 };
    let rect2 = Rectangle { width: 5, height: 8 };
    let rect3 = Rectangle { width: 60, height: 9 };

    println!("Can rect 1 hold rect 2 {}", rect1.can_hold(&rect2));
    println!("Can rect 1 hold rect 3 {}", rect1.can_hold(&rect3));
    println!("Area pf rect1 {}", rect1.area())
}
