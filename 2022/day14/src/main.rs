// --- Day 14: Regolith Reservoir ---
// The distress signal leads you to a giant waterfall! Actually, hang on - the signal seems like it's coming from the waterfall itself, and that doesn't make any sense. However, you do notice a little path that leads behind the waterfall.

// Correction: the distress signal leads you behind a giant waterfall! There seems to be a large cave system here, and the signal definitely leads further inside.

// As you begin to make your way deeper underground, you feel the ground rumble for a moment. Sand begins pouring into the cave! If you don't quickly figure out where the sand is going, you could quickly become trapped!

// Fortunately, your familiarity with analyzing the path of falling material will come in handy here. You scan a two-dimensional vertical slice of the cave above you (your puzzle input) and discover that it is mostly air with structures made of rock.

// Your scan traces the path of each solid rock structure and reports the x,y coordinates that form the shape of the path, where x represents distance to the right and y represents distance down. Each path appears as a single line of text in your scan. After the first point of each path, each point indicates the end of a straight horizontal or vertical line to be drawn from the previous point. For example:

// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9
// This scan means that there are two paths of rock; the first path consists of two straight lines, and the second path consists of three straight lines. (Specifically, the first path consists of a line of rock from 498,4 through 498,6 and another line of rock from 498,6 through 496,6.)

// The sand is pouring into the cave from point 500,0.

// Drawing rock as #, air as ., and the source of the sand as +.

// Sand is produced one unit at a time, and the next unit of sand is not produced until the previous unit of sand comes to rest. A unit of sand is large enough to fill one tile of air in your scan.

// A unit of sand always falls down one step if possible. If the tile immediately below is blocked (by rock or sand), the unit of sand attempts to instead move diagonally one step down and to the left. If that tile is blocked, the unit of sand attempts to instead move diagonally one step down and to the right. Sand keeps moving as long as it is able to do so, at each step trying to move down, then down-left, then down-right. If all three possible destinations are blocked, the unit of sand comes to rest and no longer moves, at which point the next unit of sand is created back at the source.

// So, drawing sand that has come to rest as o, the first unit of sand simply falls straight down and then stops
// The second unit of sand then falls straight down, lands on the first one, and then comes to rest to its left. Once all possible units of sand shown above have come to rest, all further sand flows out the bottom, falling into the endless void.
// Using your scan, simulate the falling sand. How many units of sand come to rest before sand starts flowing into the abyss below?

// imports
use std::fs::File;
use std::io::{BufRead, BufReader};

// main function

pub fn main() {
    // read input file
    let file = File::open("../input_data/input_14.txt").unwrap();
    let reader = BufReader::new(file);

    // create a vector of strings to hold the input
    let mut input: Vec<String> = Vec::new();

    // read the input file line by line
    for line in reader.lines() {
        // add each line to the vector
        input.push(line.unwrap());
    }

    // create a vector of vectors of chars to hold the map
    let mut map: Vec<Vec<char>> = Vec::new();

    // loop through the input
    for line in input {
        // create a vector of chars to hold the line
        let mut line_vec: Vec<char> = Vec::new();

        // loop through the line
        for c in line.chars() {
            // add each char to the line vector
            line_vec.push(c);
        }

        // add the line vector to the map
        map.push(line_vec);
    }

    // loop through the map
    for y in 0..map.len() {
        // loop through the line
        for x in 0..map[y].len() {
            // check if the current char is a sand
            if map[y][x] == '|' {
                // check if the char below is air
                if map[y + 1][x] == '.' {
                    // set the current char to air
                    map[y][x] = '.';

                    // set the char below to sand
                    map[y + 1][x] = '|';
                } else if map[y + 1][x] == '#' {
                    // check if the char below and to the left is air
                    if map[y + 1][x - 1] == '.' {
                        // set the current char to air
                        map[y][x] = '.';

                        // set the char below and to the left to sand
                        map[y + 1][x - 1] = '|';
                    } else if map[y + 1][x + 1] == '.' {
                        // check if the char below and to the right is air
                        // set the current char to air
                        map[y][x] = '.';

                        // set the char below and to the right to sand
                        map[y + 1][x + 1] = '|';
                    }
                }
            }
        }
    }

    // The solution to part 1
    let mut part1 = 0;

    // loop through the map
    for y in 0..map.len() {
        // loop through the line
        for x in 0..map[y].len() {
            // check if the current char is sand
            if map[y][x] == '|' {
                // increment the part 1 solution
                part1 += 1;
            }
        }
    }

    // print the solution to part 1

    println!("Part 1: {}", part1);
}
