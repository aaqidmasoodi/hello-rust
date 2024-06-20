use std::io;    

fn main() {


    let mut name = String::new();

    println!("Hello, What is your name? ");
    io::stdin()
        .read_line(&mut name)
        .expect("error reading input");


    println!("Hello {}, Welcome to rust.", name);
}
