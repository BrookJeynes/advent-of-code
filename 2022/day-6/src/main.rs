use std::{fs, collections::HashSet};

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    println!("Answer one: {}\nAnswer two: {}", answer_one, answer_two);
}

fn part_one(input: &String) -> i32 {
    const SEQUENCE_LEN: usize = 4;

    for checked in 0..(input.len()-SEQUENCE_LEN) {
        let range = checked..(checked + SEQUENCE_LEN);

        if input[range].chars().collect::<HashSet<char>>().len() == SEQUENCE_LEN {
            return (checked + SEQUENCE_LEN) as i32
        }
    }
    
    -1
}

fn part_two(input: &String) -> i32 {
    const SEQUENCE_LEN: usize = 14;

    for checked in 0..(input.len()-SEQUENCE_LEN) {
        let range = checked..(checked + SEQUENCE_LEN);

        if input[range].chars().collect::<HashSet<char>>().len() == SEQUENCE_LEN {
            return (checked + SEQUENCE_LEN) as i32
        }
    }
    
    -1
}
