fn own_integer(x: i32) {
    // modifi x = x+1 and print it
    let x = x + 1;
    println!("x = {}", x);
}


fn own_string(s: String) -> String {
    // modifi s = s + " world" and print it
    let s = s + " world";
    println!("s = {}", s);
    s
}

fn main() {
    let x = 5;
    //let y = own_integer(x);
    println!("x = {}", x);
    //println!("y = {}", y);

    let my_string = String::from("Hello, world!");
    println!("my_string = {}", my_string);
    let ss = own_string(my_string);
    println!("ss = {}", ss);
}