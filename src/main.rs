extern crate aoc_2018;
extern crate regex;

use regex::Regex;
use aoc_2018::aoc_input::INPUT_3;

#[derive(Debug)]
struct Rectangle {
    id: String,
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

fn main() {
    let inputs = INPUT_3.split("\n")
        .collect::<Vec<&str>>();

    let re = Regex::new(r"^#(\d*) @ (\d*),(\d*): (\d*)x(\d*)$").unwrap();
    let mut rectangles = Vec::new();

    for input in inputs {
        let captures = re.captures(input).unwrap();
        let id = captures.get(1).unwrap().as_str();
        let x: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let y: usize = captures.get(3).unwrap().as_str().parse().unwrap();
        let width: usize = captures.get(4).unwrap().as_str().parse().unwrap();
        let height: usize = captures.get(5).unwrap().as_str().parse().unwrap();
        let rectangle = Rectangle {
            id: String::from(id),
            x1: x,
            y1: y,
            x2: x + width - 1,
            y2: y + height - 1,

        };

        rectangles.push(rectangle)
    }
    println!("{:?}", rectangles);

    let mut grid = [[0; 1000]; 1000];

    for rectangle in rectangles {
        for x in rectangle.x1..=rectangle.x2 {
            for y in rectangle.y1..=rectangle.y2 {
                let c = grid[x][y];
                grid[x][y] = c + 1;
            }
        }
    }

    let mut overlaps = 0;

    for row in grid.iter() {
        for cell in row.iter() {
            if *cell > 1 {
                overlaps += 1;
            }
        }
    }

    println!("part 1: found {} overlapping rectangles", overlaps);


//    let squares = inputs;
}

fn overlap(a: Rectangle, b: Rectangle) -> bool {
    if (a.x1 <= b.x2)
        & (a.x2 >= b.x1)
        & (a.y1 >= b.y2)
        & (a.y2 <= b.y1) {
        return true;
    }

    return false;
}
