advent_of_code::solution!(2);

#[derive(Debug, Default)]
struct GameTurn {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameTurn {
    fn is_turn_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.red <= 14
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut valid_turns: u32 = 0;
    let mut games_list: Vec<Vec<GameTurn>> = Vec::new();

    let lines = input.split("\n").collect::<Vec<_>>();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let (_, turns) = line.split_once(": ").unwrap();

        let mut game_turn_list = Vec::new();

        let turns = turns.split("; ").collect::<Vec<_>>();
        for turn in turns {
            let cubes = turn.split(", ").collect::<Vec<_>>();
            let mut game_turn = GameTurn::default();

            for cube in cubes {
                let (amount, color) = cube.split_once(' ').unwrap();
                let amount: u32 = amount.parse().unwrap();

                match color {
                    "red" => game_turn.red = amount,
                    "green" => game_turn.green = amount,
                    "blue" => game_turn.blue = amount,
                    _ => panic!("Nope"),
                }
            }
            game_turn_list.push(game_turn);
        }
        games_list.push(game_turn_list);
    }

    'next_game: for (index, game) in games_list.iter().enumerate() {
        for turn in game {
            if !turn.is_turn_valid() {
                continue 'next_game;
            }
        }
        valid_turns += u32::try_from(index).unwrap() + 1;
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
