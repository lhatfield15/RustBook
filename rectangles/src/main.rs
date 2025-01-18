fn main() {
    let rect = (30, 50);

    println!("Area is {} pixels.", area(rect));
}

fn area(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}
