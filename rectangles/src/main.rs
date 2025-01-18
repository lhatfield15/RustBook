#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

fn main() {
    let rect = Rectangle { w: 30, h: 50 };

    println!("Area is {} pixels.", area(&rect));
    println!("Rect is {rect:#?}")
}

fn area(rect: &Rectangle) -> u32 {
    rect.h * rect.w
}
