mod utils;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/dive.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    // let result = calculate_depth(&input);
    let result = call_method(&input, calculate_depth_and_position);
    println!("Final depth {} and final distance {}", result.0, result.1);
    let result = call_method(&input, calculate_depth_position_and_aim);
    println!("Final depth {} and final distance {}", result.0, result.1);
}

/// This method calculates the final depth and position of a command.
/// The commands can be up which decreases the aim, down which increases the aim
/// and forward that increases the position.
fn calculate_depth_and_position(line: &str, _: &mut i32) -> (i32, u32) {
    let mut result = (0, 0);
    let mut split = line.split_whitespace();
    match split.next().unwrap() {
        "forward" => result.1 += split.next().unwrap().parse::<u32>().unwrap(),
        "up" => result.0 -= split.next().unwrap().parse::<i32>().unwrap(),
        "down" => result.0 += split.next().unwrap().parse::<i32>().unwrap(),
        _ => {}
    }
    result
}

/// This method calculates the final depth and position of a command.
/// The commands can be up which decreases the aim, down which increases the aim
/// and forward that increases the position, forward also increases aim by mutiplying
/// the increase in position and the current aim value.
fn calculate_depth_position_and_aim(line: &str, aim: &mut i32) -> (i32, u32) {
    let mut result = (0, 0);
    let mut split = line.split_whitespace();
    match split.next().unwrap() {
        "down" => *aim += split.next().unwrap().parse::<i32>().unwrap(),
        "up" => *aim -= split.next().unwrap().parse::<i32>().unwrap(),
        "forward" => {
            let int = split.next().unwrap().parse::<i32>().unwrap();
            result.0 += int * *aim;
            result.1 += int as u32;
        }
        _ => {}
    }
    result
}

/// This method calculates the final depth and position of a submarine simulated as in
/// inputs/dive.txt.
/// The file has a list of commands that simulate a submarine descent into the depths
/// of the ocean.
/// It then runs a parser of the commands that's passed as an argument that returns
/// the depth and direction of a command.
/// In the end it returns a pair of numbers representing the depth and position in
/// this order.
fn call_method<F>(file: &String, mut f: F) -> (i32, u32)
where
    F: FnMut(&str, &mut i32) -> (i32, u32),
{
    let mut result = (0, 0);
    let mut aim = 0;
    for line in file.lines() {
        let tmp = f(line, &mut aim);
        result.0 += tmp.0;
        result.1 += tmp.1;
    }
    result
}
