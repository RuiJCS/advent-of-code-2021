mod depth;
use depth::depth::{count_increased_depth,count_increased_depth_intervals};

mod utils;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/depths.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    println!("Number of increased measurements {}", count_increased_depth(&input));

    println!("Number of increased measurements in intervfals {}", count_increased_depth_intervals(&input));
}
