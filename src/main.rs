extern crate aoc_2018;

use aoc_2018::aoc_input::INPUT_3;

struct Square {
    id: String,
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn main() {
    let inputs = INPUT_3.split("\n")
        .collect::<Vec<&str>>();

    let squares = inputs;
}

fn parse_square(string: &str) -> Square {
    // "#1 @ 338,764: 20x24"
    let split = string.split(" ")
        .collect::<Vec<&str>>();

}


