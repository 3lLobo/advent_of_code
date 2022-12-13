// --- Day 13: Distress Signal ---
// You climb the hill and again try contacting the Elves. However, you instead receive a signal you weren't expecting: a distress signal.

// Your handheld device must still not be working properly; the packets from the distress signal got decoded out of order. You'll need to re-order the list of received packets (your puzzle input) to decode the message.

// Your list consists of pairs of packets; pairs are separated by a blank line. You need to identify how many pairs of packets are in the right order.

// Packet data consists of lists and integers. Each list starts with [, ends with ], and contains zero or more comma-separated values (either integers or other lists). Each packet is always a list and appears on its own line.

// When comparing two values, the first value is called left and the second value is called right. Then:

// If both values are integers, the lower integer should come first. If the left integer is lower than the right integer, the inputs are in the right order. If the left integer is higher than the right integer, the inputs are not in the right order. Otherwise, the inputs are the same integer; continue checking the next part of the input.
// If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs out of items first, the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right order. If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
// If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].

// Using these rules, you can determine which of the pairs in the example are in the right order.

// What are the indices of the pairs that are already in the right order? (The first pair has index 1, the second pair has index 2, and so on.) In the above example, the pairs in the right order are 1, 2, 4, and 6; the sum of these indices is 13.

// Determine which pairs of packets are already in the right order. What is the sum of the indices of those pairs?

//  Load the data

use std::fs;

// Load the data from input_data folder above the root of the project

pub fn load_data() -> String {
    let data = fs::read_to_string("../input_data/input_13.txt").unwrap_or_else(|_| panic!("Hulp"));
    data
}

// Parse the data

pub fn parse(data: &str) -> Vec<Vec<Vec<i32>>> {
    let mut packets = Vec::new();
    let mut packet = Vec::new();
    let mut list = Vec::new();
    let mut num = String::new();
    for c in data.chars() {
        match c {
            '[' => {
                if !num.is_empty() {
                    list.push(num.parse::<i32>().unwrap());
                    num.clear();
                }
            }
            ']' => {
                if !num.is_empty() {
                    list.push(num.parse::<i32>().unwrap());
                    num.clear();
                }
                packet.push(list);
                list = Vec::new();
            }
            ',' => {
                if !num.is_empty() {
                    list.push(num.parse::<i32>().unwrap());
                    num.clear();
                }
            }
            '1'..='9' => num.push(c),
            _ => {
                if !num.is_empty() {
                    list.push(num.parse::<i32>().unwrap());
                    num.clear();
                }
                if !list.is_empty() {
                    packet.push(list);
                    list = Vec::new();
                }
                if !packet.is_empty() {
                    packets.push(packet);
                    packet = Vec::new();
                }
            }
        }
    }
    packets.push(packet);
    packets
}

// Compare two lists

pub fn compare(left: &Vec<i32>, right: &Vec<i32>) -> bool {
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            return true;
        } else if left[i] > right[j] {
            return false;
        } else {
            i += 1;
            j += 1;
        }
    }
    if i == left.len() {
        return true;
    }
    false
}

// Compare two packets. Check packet length to avoid out of bounds errors.

pub fn compare_packets(left: &Vec<Vec<i32>>, right: &Vec<Vec<i32>>) -> bool {
    if left.len() == right.len() {
        for i in 0..left.len() {
            if !compare(&left[i], &right[i]) {
                return false;
            }
        }
        return true;
    }
    false
}

// Find the right order of the packets

pub fn find_right_order(packets: &Vec<Vec<Vec<i32>>>) -> Vec<usize> {
    let mut right_order = Vec::new();
    for i in 0..packets.len() {
        let mut j = 0;
        while j < packets.len() {
            if i != j {
                if compare_packets(&packets[i], &packets[j]) {
                    right_order.push(i);
                    break;
                }
            }
            j += 1;
        }
    }
    right_order
}

// Find the sum of the indices of the right order

pub fn find_sum(right_order: &Vec<usize>) -> usize {
    let mut sum = 0;
    for i in right_order {
        sum += i + 1;
    }
    sum
}

// Main

fn main() {
    let data = load_data();
    let packets = parse(&data);
    let right_order = find_right_order(&packets);
    let sum = find_sum(&right_order);
    println!("The sum of the indices of the right order is: {}", sum);
}

//  The sum of the indices of the right order is 2427.
