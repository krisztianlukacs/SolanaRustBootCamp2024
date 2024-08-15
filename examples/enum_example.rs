enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit message received.");
        }
        Message::Move { x, y } => {
            println!("Move to position: x = {}, y = {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to: red = {}, green = {}, blue = {}", r, g, b);
        }
    }
}

fn main() {
    let msg1 = Message::Move { x: 10, y: 20 };
    let msg2 = Message::Write(String::from("Hello, world!"));
    
    process_message(msg1);
    process_message(msg2);
}