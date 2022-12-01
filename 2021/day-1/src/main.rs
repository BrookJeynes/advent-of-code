use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    println!("Answer one: {}\nAnswer two: {}", answer_one, answer_two);
}

fn part_one(input: &String) -> i32 {
    let mut count = 0;
    let mut previous = 0;

    for line in input.lines().skip(1) {
        let line: i32 = line.parse().unwrap();

        if line > previous {
            count += 1;
        } 

        previous = line;
    }

    count
}

fn part_two(input: &String) -> i32 {
    let mut count = 0;
    let mut previous = -1;
    let input: Vec<i32> = input.lines().map(|v| v.parse().unwrap()).collect();

    for index in 0..input.len() {
        if index+2 > input.len()-1 {
            break;
        }

        let three_sum = get_sum_of_range(&input, (index, index+2));

        // fix this - should skip the first three_sum
        if three_sum > previous && previous != -1 {
            count += 1;
        } 

        previous = three_sum;
    }

    count
}

fn get_sum_of_range(items: &Vec<i32>, range: (usize, usize)) -> i32 {
    let mut slice = Vec::new();

    for (index, item) in items.iter().enumerate().skip(range.0) {
        if index == range.1+1 { break };

        slice.push(*item);
    } 

    slice.iter().sum()
}
