use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    println!("Answer one: {}\nAnswer two: {}", answer_one, answer_two);
}

fn part_one(input: &String) -> i32 {
    let elf_calories: i32 = input
        .trim()
        .split("\n\n")
        .map(|elf| 
            elf.lines()
                .map(|meal_calories|
                    meal_calories.parse::<i32>().unwrap_or_default()
                ).sum()
        ).max().unwrap();

    elf_calories
}

fn part_two(input: &String) -> i32 {
    let mut elf_calories: Vec<i32> = input
        .trim()
        .split("\n\n")
        .map(|elf| 
            elf.lines()
                .map(|meal_calories| 
                    meal_calories.parse::<i32>().unwrap_or_default()
                ).sum()
        ).collect();

    // sort list in reverse order
    elf_calories.sort_by(|a, b| b.cmp(a));
    elf_calories.iter().take(3).sum::<i32>()
}
