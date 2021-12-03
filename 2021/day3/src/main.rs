use std::fs;

pub fn load() -> String {
    let file = "data.txt";
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading data!"))
}

pub fn main() {
    let data = load();
    let n_len = data.lines().next().unwrap().len();
    println!("{}", format!("Length {}", n_len));

}