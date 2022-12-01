use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    println!("Answer one: {}\nAnswer two: {}", answer_one, answer_two);
}

fn part_one(input: &String) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let direction = line.get(0).unwrap();
        let val: i32 = line.get(1).unwrap().parse().unwrap();


        match *direction {
            "forward" => horizontal += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => continue
        }
    }

    horizontal * depth
}

fn part_two(input: &String) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let direction = line.get(0).unwrap();
        let val: i32 = line.get(1).unwrap().parse().unwrap();


        match *direction {
            "forward" => {
                if aim != 0 {
                    depth += val * aim; 
                }

                horizontal += val
            },
            "down" => aim += val,
            "up" => aim -= val,
            _ => continue
        }
    }

    horizontal * depth 
}
