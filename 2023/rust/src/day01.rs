use anyhow::Result;

const FILE: &str = "./src/data/day01.txt";

fn part_a() -> Result<u32> {
    let contents = std::fs::read_to_string(FILE)?;
    let mut total = 0;

    for line in contents.lines() {
        let first_num = line
            .chars()
            .find(|c| c.is_digit(10))
            .unwrap_or_default()
            .to_digit(10)
            .unwrap_or_default();

        let last_num = line
            .chars()
            .rev()
            .find(|c| c.is_digit(10))
            .unwrap_or_default()
            .to_digit(10)
            .unwrap_or_default();

        total += (first_num * 10) + last_num;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use crate::day01::part_a;

    #[test]
    fn day_01_part_a() {
        let total = part_a().unwrap_or(0);
        assert_eq!(55090, total);
    }
}
