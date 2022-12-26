// --- day 25: Full of Hot Air ---
// --- part 1 ---

// read the input
fn read_input() -> Vec<String> {
    let input = std::fs::read_to_string("../input_data/input_25.txt").unwrap();
    input.lines().map(|s| s.to_string()).collect()
}

// parse SNAFU
fn parse_snafu(snafu: String) -> i32 {
    let mut length: u32 = 0;
    let snafu = snafu.chars();
    // vector of chars
    let mut char_vec: Vec<char> = Vec::new();
    for c in snafu {
        char_vec.push(c);
    }
    // reverse the vector
    char_vec.reverse();
    // convert to iterator
    let snafu = char_vec.into_iter();
    let mut result = 0;
    for c in snafu {
        length += 1;
        match c {
            '1' => result += 5_i32.pow(length) * 1,
            '2' => result += 5_i32.pow(length) * 2,
            '0' => result += 5_i32.pow(length) * 0,
            '-' => result -= 5_i32.pow(length) * 1,
            '=' => result -= 5_i32.pow(length) * 2,
            _ => panic!("unknown operator"),
        }
        length -= 1;
    }
    result
}

// parse input with mod 5 and. - subtracts 1 mod 5 and = subtracts 2 mod 5.
fn get_decimals(input: &Vec<String>) -> Vec<i32> {
    input.iter().map(|s| parse_snafu(s.to_string())).collect()
}

// parse all SNAFUs to decimals and return the sum. That's the puzzle answer.
fn main1() {
    let input = read_input();
    let decimals = get_decimals(&input);
    let sum: i32 = decimals.iter().sum();
    // Part1 answer
    println!("Part1!\nThe sum in decimals is: {}", sum);
}

// --- part 2 ---

fn main2() {
    // Part2 answer
    println!("Part2!\nNot implemented 😕");
}

fn main() {
    main1();
    main2();
}
