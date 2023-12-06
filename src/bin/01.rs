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

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            Some(1)
        } else if reduced_line.starts_with("two") {
            Some(2)
        } else if reduced_line.starts_with("three") {
            Some(3)
        } else if reduced_line.starts_with("four") {
            Some(4)
        } else if reduced_line.starts_with("five") {
            Some(5)
        } else if reduced_line.starts_with("six") {
            Some(6)
        } else if reduced_line.starts_with("seven") {
            Some(7)
        } else if reduced_line.starts_with("eight") {
            Some(8)
        } else if reduced_line.starts_with("nine") {
            Some(9)
        } else {
            reduced_line.chars().next().unwrap().to_digit(10)
        };

        result
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.lines().map(process_line).sum::<u32>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}
