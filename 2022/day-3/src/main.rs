#![feature(iter_array_chunks)]
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    println!("Answer one: {}\nAnswer two: {}", answer_one, answer_two);
}

fn part_one(input: &String) -> i32 {
    let mut sum = 0;

    for collection in input.lines() {
        let (compartment_one, compartment_two) = collection.split_at(collection.len() / 2);

        for char in compartment_one.chars() {
            if compartment_two.contains(char) {
                if char.is_uppercase() {
                    sum += char as i32 - 38;
                } else {
                    sum += char as i32 - 96;
                };

                break;
            } 
        }
    }

    sum
}

fn part_two(input: &String) -> i32 {
    let mut sum = 0;

    for collections in input.lines().array_chunks::<3>() {
        let elf_one = collections.get(0).unwrap();
        let elf_two = collections.get(1).unwrap();
        let elf_three = collections.get(2).unwrap();

        for char in elf_one.chars() {
            if elf_two.contains(char) && elf_three.contains(char) {
                if char.is_uppercase() {
                    sum += char as i32 - 38;
                } else {
                    sum += char as i32 - 96;
                };

                break;
            } 
        }
    }

    sum 
}
