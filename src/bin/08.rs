use std::collections::BTreeMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

advent_of_code::solution!(8);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parser(input: &str) -> IResult<&str, (Vec<Direction>, BTreeMap<&str, (&str, &str)>)> {
    let (input, instructions) = many1(alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('L').map(|_| Direction::Left),
    )))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alpha1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alpha1, tag(", "), alpha1),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;

    Ok((input, (instructions, map)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (input, (instructions, map)) = parser(input).expect("should validly parse");

    let mut current_node = "AAA";
    let Some(step_count) =
        instructions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(index, instruction)| {
                let options = map.get(current_node).expect("always exist at a valid node");
                let next_node = match instruction {
                    Direction::Left => options.0,
                    Direction::Right => options.1,
                };
                if next_node == "ZZZ" {
                    Some(index + 1)
                } else {
                    current_node = next_node;
                    None
                }
            })
    else {
        panic!("infinite iterator can't produce None")
    };

    Some(step_count as u32)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
