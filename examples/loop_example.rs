#[warn(dead_code)]

fn fn_loop() -> i32 {
    let mut counter = 1;
    loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break;
        }
    }
    counter
}

fn main() {
    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break the loop at counter = {}.", stop_loop);
    
    println!("FN Loop: {}", fn_loop());
    
}
