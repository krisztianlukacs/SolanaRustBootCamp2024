struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Konstruktor (nem kötelező, de gyakori minta)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Metódus, amely kiszámítja a területet
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // Példány létrehozása a konstruktor használatával
    let rect = Rectangle::new(30, 50);
    println!("Area: {}", rect.area());
}