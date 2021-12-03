use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello, world!");
    let msg = "DO quick!!";
    say(msg.as_bytes(), msg.chars().count(), &mut BufWriter::new(stdout().lock())).unwrap();
}
