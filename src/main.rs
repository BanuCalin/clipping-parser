use std::env;

fn main() {
    println!("Hello, world!");
    println!("args count: {}", env::args().count());
    for arg in env::args() {
        println!("{}", arg);
    }
}