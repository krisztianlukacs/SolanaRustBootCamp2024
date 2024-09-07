trait HasArea {
    fn area(&self) -> u32;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl HasArea for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area());
}
