extern crate aoc;

use std::collections::HashSet;
use aoc::aoc_input::INPUT_1;

fn main() -> std::io::Result<()> {
    let str_nums = INPUT_1.split("\n")
        .collect::<Vec<&str>>();
    // idk why I had to split this across two vars
    let nums = str_nums.iter()
        .map(|&n| n.parse::<i32>().unwrap());

    let mut total = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    for as_int in nums.cycle() {
        total += as_int;
        if seen.contains(&total) {
            println!("saw {} twice", total);
            break;
        } else {
            seen.insert(total);
        }
    }

    Ok(())
}
