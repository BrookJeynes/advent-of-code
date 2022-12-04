use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    println!("Answer one: {}\nAnswer two: {}", answer_one, answer_two);
}

fn part_one(input: &String) -> i32 {
    let mut duplicate = 0;

    for elfs in input.lines() {
        let elfs: Vec<&str> = elfs.split(",").collect();

        let range_one: Vec<i32> = elfs
            .get(0).unwrap()
            .split("-")
            .map(|range| 
                range.parse::<i32>().unwrap()
            ).collect();

        let range_two: Vec<i32> = elfs
            .get(1).unwrap()
            .split("-")
            .map(|range| 
                range.parse::<i32>().unwrap()
            ).collect();

        if 
            (range_one.get(0).unwrap() <= range_two.get(0).unwrap() && range_one.get(1).unwrap() >= range_two.get(1).unwrap()) ||
            (range_one.get(0).unwrap() >= range_two.get(0).unwrap() && range_one.get(1).unwrap() <= range_two.get(1).unwrap())
        {
            duplicate += 1;
        }
    }

    duplicate
}

fn part_two(input: &String) -> i32 {
    let mut duplicate = 0;

    for elfs in input.lines() {
        let elfs: Vec<&str> = elfs.split(",").collect();

       let range_one: Vec<i32> = elfs
            .get(0).unwrap()
            .split("-")
            .map(|range| 
                range.parse::<i32>().unwrap()
            ).collect();

        let range_two: Vec<i32> = elfs
            .get(1).unwrap()
            .split("-")
            .map(|range| 
                range.parse::<i32>().unwrap()
            ).collect(); 

        let range_one: Vec<i32> = (*range_one.get(0).unwrap()..=*range_one.get(1).unwrap()).collect();
        let range_two: Vec<i32> = (*range_two.get(0).unwrap()..=*range_two.get(1).unwrap()).collect();

        for val in range_one {
            if range_two.contains(&val) {
                duplicate += 1;
                break
            }
        }
    }

    duplicate
}
