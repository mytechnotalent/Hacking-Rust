use std::{thread, time::Duration};

fn main() {
    let index = 0;

    if index == 0 {
        println!("I am zero!");
    } else if index == 1 {
        println!("I am one!");
    } else {
        println!("I am nothing!");
    }

    thread::sleep(Duration::from_millis(4000));

    loop {
        println!("forever print!");
    }
}
