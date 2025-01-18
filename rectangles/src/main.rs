#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }
}

fn main() {
    let rect = Rectangle { w: 30, h: 50 };

    println!("The area is {}", rect.area());
}
