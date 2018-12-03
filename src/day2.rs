extern crate aoc;

use aoc::aoc_input::INPUT_2;
use std::collections::HashMap;

fn main() {
    let inputs = INPUT_2.split("\n")
        .collect::<Vec<&str>>();

    let mut twos = 0;
    let mut threes = 0;

    for s in inputs.clone() {
        let (two, three) = counts(s);
        if two {
            twos += 1;
        }
        if three {
            threes += 1;
        }
    }

    println!("twos: {}, threes: {}, total: {}", twos, threes, twos * threes);


    for s in inputs.clone() {
        for t in inputs.clone() {
            if count_differences(s, t) == 1 {
                println!("found difference of 1:\n{}\n{}", s, t);
                println!("common chars: {}", remove_difference(s, t));
                return
            }
        }
    }
}


fn counts(string: &str) -> (bool, bool) {
    let mut counts = HashMap::new();

    for c in string.chars() {
        let current_count = *counts.entry(c).or_insert(0);
        counts.insert(c, current_count + 1);
    }

    let mut two = false;
    let mut three = false;

    for (_, count) in &counts {
        if *count == 2 {
            two = true;
        }
        if *count == 3 {
            three = true;
        }
    }

    return (two, three);
}

fn count_differences(string: &str, compare_to: &str) -> i32 {
    let mut diffs = 0;

    for (orig, other) in string.chars().zip(compare_to.chars()) {
        if orig != other {
            diffs += 1
        }
    }

    return diffs;
}

fn remove_difference(s1: &str, s2: &str) -> String {
    let mut vec  = Vec::new();
    for (orig, other) in s1.chars().zip(s2.chars()) {
        if orig == other {
            vec.push(orig);
        }
    }

    return vec.into_iter().collect();
}
