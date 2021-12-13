use std::fs;
use std::io;


fn main() {
    println!("Hello, world!");

    let mut day = String::new();
    println!("What day is it?");
    io::stdin()
        .read_line(&mut day)
        .expect("nope!");
    
    let data = read_data(day);
    let board = Board::parse(data);
    print!("{:#?}", board.vals)

}

pub fn read_data(day: String) -> String {
    let data = fs::read_to_string("../input_data/input_11.txt").unwrap_or_else(|_| panic!("Hulp"));
    data
}


#[derive(Debug, Clone)]
struct Board {
    vals: Vec<Vec<u8>>,
    checked: Vec<Vec<bool>>,
}


impl Board {
    
    fn parse(data: String) -> Board {
        let mut vals = Vec::new();
        let mut checked = Vec::new();

        for row in data.lines() {
            checked.push(vec![false; row.len()]);
            let mut val = Vec::new();

            for element in row.chars() {
                let octopus = element.to_digit(10).unwrap() as u8;
                val.push(octopus.clone());
            }
            vals.push(val.clone());

        }
        let board = Board {vals, checked};
        board
    }
}