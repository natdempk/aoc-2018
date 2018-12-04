extern crate aoc_2018;
extern crate regex;

use regex::Regex;
use aoc_2018::aoc_input::INPUT_3;

#[derive(Eq, Debug, Clone)]
struct Rectangle {
    id: String,
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        return self.id.eq(&other.id)
            & self.x1.eq(&other.x1)
            & self.x2.eq(&other.x2)
            & self.y1.eq(&other.y1)
            & self.y2.eq(&other.y2);
    }
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

    let mut grid = [[0; 1000]; 1000];

    for rectangle in rectangles.clone() {
        for x in rectangle.x1..=rectangle.x2 {
            for y in rectangle.y1..=rectangle.y2 {
                let c = grid[x][y];
                grid[x][y] = c + 1;
            }
        }
    }

    let mut overlapping_cells = 0;

    for row in grid.iter() {
        for cell in row.iter() {
            if *cell > 1 {
                overlapping_cells += 1;
            }
        }
    }

    println!("part 1: found {} overlapping rectangles", overlapping_cells);

    for rect in rectangles.clone() {
        let mut overlaps_another = false;
        for other in rectangles.clone() {
            if !rect.clone().eq(&other)
                & overlaps(rect.clone(), other) {
                overlaps_another = true;
            }
        }

        if !overlaps_another {
            println!("part 2: rectangle with id {} overlaps no other", rect.id);
        }
    }
}

fn overlaps(a: Rectangle, b: Rectangle) -> bool {
    if (a.x1 > b.x2)
        | (b.x1 > a.x2)
        | (a.y1 > b.y2)
        | (b.y1 > a.y2) {
        return false;
    }

    return true;
}
