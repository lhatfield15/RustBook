#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        w: dbg!(30 * scale),
        h: 50,
    };

    dbg!(&rect);
    println!("Area is {} pixels.", area(&rect));
    println!("Rect is {rect:#?}")
}

fn area(rect: &Rectangle) -> u32 {
    rect.h * rect.w
}
