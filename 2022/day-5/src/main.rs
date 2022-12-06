use std::{fs, collections::HashMap};

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    println!("Answer one: {}\nAnswer two: {}", answer_one, answer_two);
}

fn part_one(input: &String) -> String {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut tower_tops = String::new();

    let mut towers: HashMap<u8, Vec<char>> = HashMap::from([
        (1, vec!['B','V','S','N','T','C','H','Q']),
        (2, vec!['W','D','B','G']),
        (3, vec!['F','W','R','T','S','Q', 'B']),
        (4, vec!['L','G','W','S', 'Z', 'J', 'D', 'N']),
        (5, vec!['M','P','D','V','F']),
        (6, vec!['F','W','J']),
        (7, vec!['L','N','Q','B','J','V']),
        (8, vec!['G','T','R', 'C', 'J', 'Q', 'S', 'N']),
        (9, vec!['J','S','Q','C','W','D','M']),
    ]);

    for line in input.get(1).unwrap().lines() {
        let values: Vec<&str> = line.split_whitespace().collect();

        let item_count = values.get(1).unwrap().parse::<u8>().unwrap();
        let tower_one = values.get(3).unwrap().parse::<u8>().unwrap();
        let tower_two = values.get(5).unwrap().parse::<u8>().unwrap();

        for _ in 0..item_count {
            let pop_tower = towers.get_mut(&tower_one).unwrap();

            let popped_item = pop_tower.pop().unwrap();

            let push_tower = towers.get_mut(&tower_two).unwrap();

            push_tower.push(popped_item);
        }
    } 

    // Loop over each tower and get the last entry
    for index in 1..=9 {
        let contents = towers.get(&index).unwrap();

        tower_tops.push(*contents.last().unwrap());
    }

    tower_tops
}

fn part_two(input: &String) -> String {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut tower_tops = String::new();

    let mut towers: HashMap<u8, Vec<char>> = HashMap::from([
        (1, vec!['B','V','S','N','T','C','H','Q']),
        (2, vec!['W','D','B','G']),
        (3, vec!['F','W','R','T','S','Q', 'B']),
        (4, vec!['L','G','W','S', 'Z', 'J', 'D', 'N']),
        (5, vec!['M','P','D','V','F']),
        (6, vec!['F','W','J']),
        (7, vec!['L','N','Q','B','J','V']),
        (8, vec!['G','T','R', 'C', 'J', 'Q', 'S', 'N']),
        (9, vec!['J','S','Q','C','W','D','M']),
    ]);

    for line in input.get(1).unwrap().lines() {
        let values: Vec<&str> = line.split_whitespace().collect();

        let item_count = values.get(1).unwrap().parse::<u8>().unwrap();
        let tower_one = values.get(3).unwrap().parse::<u8>().unwrap();
        let tower_two = values.get(5).unwrap().parse::<u8>().unwrap();

        for index in (0..item_count).rev() {
            let pop_tower = towers.get_mut(&tower_one).unwrap();
            let item = (pop_tower.len() - 1) - index as usize;

            let popped_item = pop_tower.remove(item);

            let push_tower = towers.get_mut(&tower_two).unwrap();

            push_tower.push(popped_item);
        }
    } 

    // Loop over each tower and get the last entry
    for index in 1..=9 {
        let contents = towers.get(&index).unwrap();

        tower_tops.push(*contents.last().unwrap());
    }

    tower_tops
}
