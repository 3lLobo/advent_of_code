use std::fs;
use std::io;


fn main() {
    println!("Hello, world!");

    let mut day = String::new();
    // println!("What day is it?");
    // io::stdin()
    //     .read_line(&mut day)
    //     .expect("nope!");
    
    let data = read_data(day);
    let mut board = Board::parse(data);
    println!("{:?}", &board);

    let rounds: usize = 100;
    for r in 1..=rounds {

        let last_flashes = board.flash_count.clone();
        board = Game::play(board.clone(), 1);
        println!("Finished round {} with {} flashes!", &r, { board.flash_count - last_flashes });

    }
    println!("Day11: Final octopus flash count {} during {} rounds", &board.flash_count, &rounds)

}

pub fn read_data(day: String) -> String {
    let data = fs::read_to_string("../input_data/input_11.txt").unwrap_or_else(|_| panic!("Hulp"));
    data
}


#[derive(Debug, Clone)]
struct Board {
    flash_count: u32,
    vals: Vec<Vec<u8>>,
    check: Vec<Vec<bool>>,
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
        let board = Board {vals: vals, check: checked, flash_count: 0};
        board
    }

    fn reset_check(&mut self) {

        let n_rows = self.vals.len();
        let n_cols = self.vals.len();

        for i_row in 0..n_rows {
            self.check[i_row] = vec![false; n_cols];
        }
    }

    // fn print_board(&mut self) {

    //     let mut to_print = String::new();
    //     to_print = "\n";

    //     for row in self.vals.iter() {
    //         for val in row.iter() {
    //             to_print.push_str(&(val.as_char()));

    //         }
    //         to_print.push_str("\n");
            
    //     }
    //     println!("{}", &round, to_print);
    // }

    fn get_range(val: usize) -> (usize, usize) {
        
        // let mut start: i8 = *val as i8 - 1;

        // let mut stop: i8 = *val as i8 + 1;
        // match val {
        //     // first occurence in grid
        //     0 =>  start = 0,
        //     // last position in grif
        //     9 => stop = 9,
        //     // catch all
        //     _ => println!(""),
        // }

        let start = if val==0 { 0 } else { val - 1 };
        let stop = if val==9 { 9 } else { val + 1 };

        (start, stop)
        }



    fn flash(&mut self, i_row: &usize, i_col: &usize) {

        let (i_row, i_col) = (i_row.clone(), i_col.clone());

        if self.check[i_row][i_col] {
            // println!("Octopus at x{} y{} alredy flashed this round!", &i_row, &i_col);

        } else {
            self.check[i_row][i_col] = true;
            println!("Flashing x{} y{}", &i_row, &i_col);

            let (x_start, x_stop) = Board::get_range(i_row);
            let (y_start, y_stop) = Board::get_range(i_col);

            for x in x_start..=x_stop {
                for y in y_start..=y_stop {              
                    if self.check[x][y] {
                        // println!("The octopus at x{} y{} lays low till the end of this round!", &x, &y);

                    } else {
                        self.vals[x][y] += 1;
                        
                        if self.vals[x][y] > 9 {
                            self.vals[x][y] = 0;
                            self.flash_count += 1;
                            self.flash(&x, &y);
                        }
                    }
                    
                }
            
                
            }
        }
    }
}


#[derive(Debug, Clone)]
struct Game {
    board: Board
}


impl Game {

    fn play(board: Board, rounds: usize) -> Board {

        let mut board = board.clone();
        board.reset_check();
        let mut init_round: bool = true;

        for _ in 0..2 {
            for (i_row, row) in board.clone().vals.iter().enumerate() {
                for (i_col, col) in row.iter().enumerate() {
                    
                    if init_round {
                        board.vals[i_row.clone()][i_col.clone()] += 1;
                    } else {
                        if col.clone() > 9 {
                            board.vals[i_row.clone()][i_col.clone()] = 0;
                            board.flash(&i_row, &i_col);
                        }
                    }
                }
            }
            init_round = false;
        }
        // println!("In {} rounds, the octopuses flashed {} times!", &rounds, &board.flash_count);

        board
    }
}

