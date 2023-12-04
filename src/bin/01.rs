advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let output = input
        .lines()
        .map(|line| {
            let mut iter = line.chars();

            let first_num = iter
                .find_map(|character| character.to_digit(10))
                .expect("character is not a number");

            let last_num = iter
                .rfind(|character| character.is_ascii_digit())
                .map(|character| character.to_digit(10).unwrap())
                .unwrap_or(first_num);

            first_num * 10 + last_num
        })
        .sum::<u32>();

    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
