// --- Day 24: Blizzard Basin ---
// With everything replanted for next year (and with elephants and monkeys to tend the grove), you and the Elves leave for the extraction point.

// Partway up the mountain that shields the grove is a flat, open area that serves as the extraction point. It's a bit of a climb, but nothing the expedition can't handle.

// At least, that would normally be true; now that the mountain is covered in snow, things have become more difficult than the Elves are used to.

// As the expedition reaches a valley that must be traversed to reach the extraction site, you find that strong, turbulent winds are pushing small blizzards of snow and sharp ice around the valley. It's a good thing everyone packed warm clothes! To make it across safely, you'll need to find a way to avoid them.

// Fortunately, it's easy to see all of this from the entrance to the valley, so you make a map of the valley and the blizzards (your puzzle input). For example:

// #.#####
// #.....#
// #>....#
// #.....#
// #...v.#
// #.....#
// #####.#
// The walls of the valley are drawn as #; everything else is ground. Clear ground - where there is currently no blizzard - is drawn as .. Otherwise, blizzards are drawn with an arrow indicating their direction of motion: up (^), down (v), left (<), or right (>).

// The above map includes two blizzards, one moving right (>) and one moving down (v). In one minute, each blizzard moves one position in the direction it is pointing:

// #.#####
// #.....#
// #.>...#
// #.....#
// #.....#
// #...v.#
// #####.#
// Due to conservation of blizzard energy, as a blizzard reaches the wall of the valley, a new blizzard forms on the opposite side of the valley moving in the same direction. After another minute, the bottom downward-moving blizzard has been replaced with a new downward-moving blizzard at the top of the valley instead:

// #.#####
// #...v.#
// #..>..#
// #.....#
// #.....#
// #.....#
// #####.#
// Because blizzards are made of tiny snowflakes, they pass right through each other. After another minute, both blizzards temporarily occupy the same position, marked 2:

// #.#####
// #.....#
// #...2.#
// #.....#
// #.....#
// #.....#
// #####.#
// After another minute, the situation resolves itself, giving each blizzard back its personal space:

// #.#####
// #.....#
// #....>#
// #...v.#
// #.....#
// #.....#
// #####.#
// Finally, after yet another minute, the rightward-facing blizzard on the right is replaced with a new one on the left facing the same direction:

// #.#####
// #.....#
// #>....#
// #.....#
// #...v.#
// #.....#
// #####.#
// This process repeats at least as long as you are observing it, but probably forever.

// Here is a more complex example:

// #.######
// #>>.<^<#
// #.<..<<#
// #>v.><>#
// #<^v^^>#
// ######.#
// Your expedition begins in the only non-wall position in the top row and needs to reach the only non-wall position in the bottom row. On each minute, you can move up, down, left, or right, or you can wait in place. You and the blizzards act simultaneously, and you cannot share a position with a blizzard.

// In the above example, the fastest way to reach your goal requires 18 steps. Drawing the position of the expedition as E, one way to achieve this is:

// Initial state:
// #E######
// #>>.<^<#
// #.<..<<#
// #>v.><>#
// #<^v^^>#
// ######.#

// Minute 1, move down:
// #.######
// #E>3.<.#
// #<..<<.#
// #>2.22.#
// #>v..^<#
// ######.#

// Minute 2, move down:
// #.######
// #.2>2..#
// #E^22^<#
// #.>2.^>#
// #.>..<.#
// ######.#

// Minute 3, wait:
// #.######
// #<^<22.#
// #E2<.2.#
// #><2>..#
// #..><..#
// ######.#

// Minute 4, move up:
// #.######
// #E<..22#
// #<<.<..#
// #<2.>>.#
// #.^22^.#
// ######.#

// Minute 5, move right:
// #.######
// #2Ev.<>#
// #<.<..<#
// #.^>^22#
// #.2..2.#
// ######.#

// Minute 6, move right:
// #.######
// #>2E<.<#
// #.2v^2<#
// #>..>2>#
// #<....>#
// ######.#

// Minute 7, move down:
// #.######
// #.22^2.#
// #<vE<2.#
// #>>v<>.#
// #>....<#
// ######.#

// Minute 8, move left:
// #.######
// #.<>2^.#
// #.E<<.<#
// #.22..>#
// #.2v^2.#
// ######.#

// Minute 9, move up:
// #.######
// #<E2>>.#
// #.<<.<.#
// #>2>2^.#
// #.v><^.#
// ######.#

// Minute 10, move right:
// #.######
// #.2E.>2#
// #<2v2^.#
// #<>.>2.#
// #..<>..#
// ######.#

// Minute 11, wait:
// #.######
// #2^E^2>#
// #<v<.^<#
// #..2.>2#
// #.<..>.#
// ######.#

// Minute 12, move down:
// #.######
// #>>.<^<#
// #.<E.<<#
// #>v.><>#
// #<^v^^>#
// ######.#

// Minute 13, move down:
// #.######
// #.>3.<.#
// #<..<<.#
// #>2E22.#
// #>v..^<#
// ######.#

// Minute 14, move right:
// #.######
// #.2>2..#
// #.^22^<#
// #.>2E^>#
// #.>..<.#
// ######.#

// Minute 15, move right:
// #.######
// #<^<22.#
// #.2<.2.#
// #><2>E.#
// #..><..#
// ######.#

// Minute 16, move right:
// #.######
// #.<..22#
// #<<.<..#
// #<2.>>E#
// #.^22^.#
// ######.#

// Minute 17, move down:
// #.######
// #2.v.<>#
// #<.<..<#
// #.^>^22#
// #.2..2E#
// ######.#

// Minute 18, move down:
// #.######
// #>2.<.<#
// #.2v^2<#
// #>..>2>#
// #<....>#
// ######E#
// What is the fewest number of minutes required to avoid the blizzards and reach the goal?

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

use progress_bar::{
    finalize_progress_bar, init_progress_bar, set_progress_bar_action,
    set_progress_bar_progression, Color, Style,
};

//  Class for the blizzards
pub struct Blizzards {
    position: (i32, i32),
    direction: char,
}

impl Blizzards {
    pub fn new(position: (i32, i32), direction: char) -> Blizzards {
        Blizzards {
            position: position,
            direction: direction,
        }
    }
    pub fn get_position(&self, minutes: i32) -> (i32, i32) {
        //  Get the position of the blizzard after the given number of minutes
        let (row, col) = self.position;
        match self.direction {
            '>' => (row, col + minutes),
            '<' => (row, col - minutes),
            '^' => (row - minutes, col),
            'v' => (row + minutes, col),
            _ => panic!("Invalid direction"),
        }
    }
}

//  read the input data
fn read_data(input_data_path: &str) -> HashMap<(i32, i32), char> {
    let path = Path::new(input_data_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let mut row = 0;
    let mut map = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut col = 0;
        for c in line.chars() {
            map.insert((row, col), c);
            col += 1;
        }
        row += 1;
    }
    map
}

//  get the boarders
fn get_boarders(map: &HashMap<(i32, i32), char>) -> (i32, i32, i32, i32) {
    let mut min_row = 0;
    let mut max_row = 0;
    let mut min_col = 0;
    let mut max_col = 0;
    for (key, value) in map.iter() {
        if value == &'#' {
            if key.0 < min_row {
                min_row = key.0;
            }
            if key.0 > max_row {
                max_row = key.0;
            }
            if key.1 < min_col {
                min_col = key.1;
            }
            if key.1 > max_col {
                max_col = key.1;
            }
        }
    }
    (min_row, max_row, min_col, max_col)
}

//  get the start
fn get_start(map: &HashMap<(i32, i32), char>, boarders: &(i32, i32, i32, i32)) -> (i32, i32) {
    // check for . in the first row
    for (key, value) in map.iter() {
        if key.0 == boarders.0 && value == &'.' {
            return key.clone();
        }
    }
    panic!("No start found");
}

//  get the target
fn get_target(map: &HashMap<(i32, i32), char>, boarders: &(i32, i32, i32, i32)) -> (i32, i32) {
    // check for . in the last row
    for (key, value) in map.iter() {
        if key.0 == boarders.1 && value == &'.' {
            return key.clone();
        }
    }
    panic!("No target found");
}

fn get_empty_map(map: &HashMap<(i32, i32), char>) -> HashMap<(i32, i32), char> {
    let mut empty_map = HashMap::new();
    for (key, value) in map.iter() {
        if value == &'#' {
            empty_map.insert(key.clone(), '#');
        } else {
            empty_map.insert(key.clone(), '.');
        }
    }
    empty_map
}

// Class for the valley
struct Valley {
    map: HashMap<(i32, i32), char>,
    boarders: (i32, i32, i32, i32),
    start: (i32, i32),
    target: (i32, i32),
    empty_map: HashMap<(i32, i32), char>,
    blizzards: Vec<Blizzards>,
    explored: HashMap<(i32, i32, i32), bool>,
    valley_maps: Vec<HashMap<(i32, i32), char>>,
}

impl Valley {
    pub fn new(input_data: &str) -> Valley {
        let map = read_data(input_data);
        let boarders = get_boarders(&map);
        let start = get_start(&map, &boarders);
        let target = get_target(&map, &boarders);
        let empty_map = get_empty_map(&map);
        let valley = Valley {
            map: map.clone(),
            boarders: boarders,
            start: start,
            target: target,
            empty_map: empty_map,
            blizzards: Vec::new(),
            explored: HashMap::new(),
            valley_maps: Vec::from([map]),
        };
        valley
    }

    fn flip_start_target(&mut self) {
        let start = self.start;
        let target = self.target;
        self.start = target;
        self.target = start;
    }

    //  create the blizzards
    fn create_blizzards(&mut self) {
        // Array of blizzards
        let mut blizzards = Vec::new();
        //  Loop through the map
        for (key, value) in self.map.iter() {
            //  If the value is a blizzard
            if value == &'>' || value == &'<' || value == &'^' || value == &'v' {
                //  Create a new blizzard
                let blizzard = Blizzards::new(key.clone(), value.clone());
                //  Add the blizzard to the array
                blizzards.push(blizzard);
            }
        }
        self.blizzards = blizzards;
    }

    fn is_within_boarders(&self, position: (i32, i32)) -> bool {
        if position.0 > self.boarders.0
            && position.0 < self.boarders.1
            && position.1 > self.boarders.2
            && position.1 < self.boarders.3
        {
            return true;
        }
        false
    }

    // Blizzard wall run
    fn wall_run(&self, position: (i32, i32), direction: char) -> (i32, i32) {
        //  Get the new position
        let mut new_position = position;
        //  Check if position is inside the boarders
        if self.is_within_boarders(position) {
            //  Check if the new position is a wall
            if self.map.get(&new_position) == Some(&'#') {
                //  Return the old position
                return position;
            }
        }
        let inner_map_length = &self.boarders.1 - 1;
        let inner_map_width = &self.boarders.3 - 1;
        // Else loop and add/subtract the map dimension until the new position is within the boarders
        while !self.is_within_boarders(new_position) {
            //  Subtract the map dimension
            match direction {
                '>' => new_position.1 -= inner_map_width,
                '<' => new_position.1 += inner_map_width,
                '^' => new_position.0 += inner_map_length,
                'v' => new_position.0 -= inner_map_length,
                _ => (),
            }
        }
        //  Return the new position
        new_position
    }

    // get the valley map for a given number of minutes
    pub fn get_valley(&mut self, minutes: i32) -> HashMap<(i32, i32), char> {
        // Check for the last saved map in the history array
        if minutes < self.valley_maps.len() as i32 {
            //  Return the map
            return self.valley_maps[minutes as usize].clone();
        }
        //  Else create a new map
        let mut valley_map = self.empty_map.clone();
        //  Loop through the blizzards
        for blizzard in self.blizzards.iter() {
            //  Get the position of the blizzard after the given number of minutes
            let position = blizzard.get_position(minutes);
            //  Get the new position of the blizzard
            let new_position = self.wall_run(position.clone(), blizzard.direction);
            //  Set the new position to a blizzard
            valley_map.insert(new_position, blizzard.direction);
        }
        //  Add the map to the history array
        self.valley_maps.push(valley_map.clone());
        //  Return the map
        valley_map
    }

    // Add postion to explored
    pub fn put_explored(&mut self, position: (i32, i32), minutes: i32) {
        self.explored
            .insert((position.0, position.1, minutes), true);
    }

    // Print the map with a path
    pub fn print_map(&mut self, positions: &Vec<(i32, i32)>) {
        let mut minutes = 0;
        let mut file = File::create("map.txt").unwrap();
        //  Loop through the path
        for position in positions.iter() {
            //  Get the map for the given number of minutes
            let mut map = self.get_valley(minutes);
            map.insert(position.clone(), 'X');

            // Write to file. Loop over length and width
            for row in 0..self.boarders.1 + 1 {
                for col in 0..self.boarders.3 + 1 {
                    let value = map.get(&(row, col));
                    match value {
                        Some(value) => {
                            file.write_all(value.to_string().as_bytes()).unwrap();
                        }
                        None => {
                            file.write_all(" ".as_bytes()).unwrap();
                        }
                    }
                }
                file.write_all("\n".as_bytes()).unwrap();
            }
            file.write_all("\n\n".as_bytes()).unwrap();
            minutes += 1;
        }
    }
}

// Class for the path nodes
#[derive(PartialEq, Clone, Debug)]
pub struct PathNode {
    position: (i32, i32),
    minutes: i32,
    parent: Option<Box<PathNode>>,
    is_final: bool,
    is_start: bool,
}

impl PathNode {
    pub fn new(
        position: (i32, i32),
        minutes: i32,
        parent: Option<Box<PathNode>>,
        target: (i32, i32),
    ) -> PathNode {
        let is_start = &parent == &None;
        let is_final = &position == &target;
        let new_node = PathNode {
            position: position,
            minutes: minutes,
            parent: parent,
            is_final,
            is_start,
        };
        new_node
    }

    // fn is_final(&mut self: PathNode, target: (i32, i32)) -> bool {
    //     self.position == target
    // }

    // Check for possible next moves
    pub fn get_next_moves(
        &self,
        next_valley_map: HashMap<(i32, i32), char>,
        target: (i32, i32),
    ) -> Vec<PathNode> {
        //  Array of next moves
        let mut next_moves = Vec::new();
        //  Get the position of the node
        let (row, col) = self.position;
        //  Get the possible moves
        let possible_moves = [
            (row, col),
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ];
        //  Loop through the possible moves
        for possible_move in possible_moves.iter() {
            //  Check if the move is a wall
            match next_valley_map.get(possible_move) {
                Some('.') => {
                    //  Else add the move to the next moves array
                    next_moves.push(PathNode::new(
                        possible_move.clone(),
                        self.minutes + 1,
                        Some(Box::new(self.clone())),
                        target,
                    ));
                }
                _ => continue,
            }
        }
        //  Return the next moves array
        next_moves
    }

    //  Get the path
    pub fn get_path(&self) -> Vec<(i32, i32)> {
        //  Array of the path
        let mut path = Vec::new();
        //  Get the current node
        let mut current_node = Some(Box::new(self.clone()));
        //  Loop through the nodes
        while current_node != None {
            //  Get the current node
            let node = current_node.unwrap();
            //  Add the position of the node to the path
            path.push(node.position);
            //  Set the current node to the parent of the current node
            current_node = node.parent;
        }
        //  Reverse the path
        path.reverse();
        //  Return the path
        path
    }
}

// Lee algorithm for a moving maze
fn blizzard_lee_algo(valley: &mut Valley, start_minutes: &i32) -> Vec<(i32, i32)> {
    // Start at time 0
    let mut minutes = start_minutes.clone();
    //  Create the start node
    let start_node = PathNode::new(valley.start, minutes, Option::None, valley.target);
    //  Create the queue
    let mut queue = VecDeque::new();
    //  Add the start node to the queue
    queue.push_back(start_node);
    let mut loop_count: u32 = 0;
    // Progress bar
    let progress_max = valley.boarders.1 * valley.boarders.3;
    init_progress_bar(progress_max as usize);
    set_progress_bar_action("Snacqs 😋", Color::Cyan, Style::Bold);
    //  Loop through the queue
    while !queue.is_empty() {
        //  Get the current node
        let current_node = queue.pop_front().expect("REASON");
        minutes = current_node.minutes;
        //  Check if the current node is the target
        if current_node.is_final {
            finalize_progress_bar();
            //  Return the path
            return current_node.get_path();
        }
        //  Get the valley map for the current time step
        let valley_map = valley.get_valley(minutes.clone());
        //  Get the next moves
        let mut next_moves = current_node.get_next_moves(valley_map, valley.target);
        //  Loop through the next moves
        while !next_moves.is_empty() {
            //  Get the next move
            let next_move = next_moves.pop().expect("REASON");
            //  Check if the position was already visited
            if valley.explored.get(&(
                next_move.position.0,
                next_move.position.1,
                next_move.minutes,
            )) == Some(&true)
            {
                //  Continue to the next move
                continue;
            } else {
                //  Else set the position to visited
                valley.put_explored(next_move.position, next_move.minutes);
                //  Add the move to the queue
                queue.push_back(next_move);
            }
        }
        // Print status
        loop_count += 1;
        // println!(
        //     "Minutes: {} Queue: {} Loops: {}",
        //     minutes,
        //     queue.len(),
        //     loop_count
        // );
        // print_progress_bar_info(
        //     "Minutes:",
        //     &minutes.to_string(),
        //     Color::LightBlue,
        //     Style::Normal,
        // );
        // print_progress_bar_info(
        //     "Queue:",
        //     &queue.len().to_string(),
        //     Color::LightBlue,
        //     Style::Normal,
        // );
        // let progress = get_progress(&loop_count, &minutes);
        set_progress_bar_progression(minutes as usize);
        // set_progress_bar_max(loop_count as usize)
    }
    finalize_progress_bar();
    //  Return an empty array
    Vec::new()
}

// fn get_progress(loop_count: &u32, minutes: &i32) -> u32 {
//     let divident = loop_count.clone() + minutes.clone() as u32;
//     let progress = (loop_count.clone() as f32 / divident as f32) * 100.0;
//     progress as u32
// }

//  Main function
fn main() {
    //  Get the input
    let input_path: &str = "../input_data/input_24.txt";
    //  Create the valley
    let mut valley = Valley::new(input_path);
    valley.create_blizzards();
    //  Get the path
    let positions = blizzard_lee_algo(&mut valley, &0);
    //  Print the path
    println!("Path: {:?}", &positions);
    //  Print the answer to part1
    let steps1 = &positions.len() - 2;
    println!("Steps to get through the Blizzard maze: {}", &steps1);
    // Save Path
    valley.print_map(&positions);

    // Part 2
    let mut minutes = steps1 as i32 + 1;
    valley.flip_start_target();

    let positions_bacq = blizzard_lee_algo(&mut valley, &minutes);
    //
    let steps2 = &positions_bacq.len() - 1;
    println!("Steps to go bacq through the Blizzard maze: {}", &steps2);

    valley.flip_start_target();
    minutes += steps2 as i32;
    let positions_again = blizzard_lee_algo(&mut valley, &minutes);
    let steps3 = &positions_again.len() - 1;
    println!("Steps to get through the Blizzard maze again: {}", &steps3);
    //  Print the answer to part2
    let steps = steps1 + steps2 + steps3;
    println!("Total steps in the Blizzard maze: {}", &steps);
}
