fn own_integer(x: i32) {
    x += 1;
    println!("x = {}", x);
}

fn main() {
    let x = 5;
    own_integer(x);
    println!("x = {}", x);
}