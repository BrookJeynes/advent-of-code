use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    println!("Answer one: {}\nAnswer two: {}", answer_one, answer_two);
}

enum Move {
    Rock,
    Paper,
    Scissors
}

fn return_move(char: &str) -> Move {
    match char {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!()
    }
}

fn part_one(input: &String) -> i32 {
    let mut total_sum = 0;

    for move_set in input.lines() {
        if let Some((opponent, you)) = move_set.split_once(' ') {
            let opponent = return_move(opponent);
            let you = return_move(you);

            let round_score = match you {
                Move::Rock => 1 + match opponent {
                    Move::Rock => 3,
                    Move::Paper => 0,
                    Move::Scissors => 6,
                },
                Move::Paper => 2 + match opponent {
                    Move::Rock => 6,
                    Move::Paper => 3,
                    Move::Scissors => 0,
                },
                Move::Scissors => 3 + match opponent {
                    Move::Rock => 0,
                    Move::Paper => 6,
                    Move::Scissors => 3,
                },
            };

            total_sum += round_score;
        }
    }

    total_sum
}

fn part_two(input: &String) -> i32 {
    let mut total_sum = 0;

    for move_set in input.lines() {
        if let Some((opponent, you)) = move_set.split_once(' ') {
            let opponent = return_move(opponent);

            let round_score = match you {
                "X" => match opponent {
                    Move::Rock => 3 + 0,
                    Move::Paper => 1 + 0,
                    Move::Scissors => 2 + 0,
                },
                "Y" => match opponent {
                    Move::Rock => 1 + 3,
                    Move::Paper => 2 + 3,
                    Move::Scissors => 3 + 3,
                },
               "Z" => match opponent {
                    Move::Rock => 2 + 6,
                    Move::Paper => 3 + 6,
                    Move::Scissors => 1 + 6,
                },
                _ => panic!()
            };

            total_sum += round_score;
        }
    }

    total_sum
}
