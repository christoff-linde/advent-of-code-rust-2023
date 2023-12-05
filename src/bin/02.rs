advent_of_code::solution!(2);

#[derive(Debug, Default)]
struct GameTurn {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameTurn {
    fn is_turn_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.red <= 14
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut valid_turns: u32 = 0;
    let lines = input.split("\n").collect::<Vec<_>>();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let (game_id, turns) = line.split_once(": ").unwrap();
        let (_, game_index) = game_id.split_once(" ").unwrap();
        let game_index: u32 = game_index.parse().unwrap();

        let turns = turns.split("; ").collect::<Vec<_>>();

        for turn in turns {
            let cubes = turn.split(", ").collect::<Vec<_>>();
            let mut game_turn = GameTurn::default();

            for cube in cubes {
                let (amount, color) = cube.split_once(' ').unwrap();
                let amount: usize = amount.parse().unwrap();

                match color {
                    "red" => game_turn.red = amount,
                    "green" => game_turn.green = amount,
                    "blue" => game_turn.blue = amount,
                    _ => panic!("Nope"),
                }
            }

            if game_turn.is_turn_valid() {
                dbg!(&game_index);
                valid_turns += (game_index + 1);
            }
        }
    }
    Some(valid_turns)
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
