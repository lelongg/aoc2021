use super::aoc;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
#[display(style = "lowercase")]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Display, FromStr, Debug)]
#[display("{direction} {magnitude}")]
struct Command {
    direction: Direction,
    magnitude: i32,
}

#[derive(Default, Debug, Copy, Clone)]
struct Position {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn apply_command(mut self, command: Command) -> Self {
        use Direction::*;
        match command.direction {
            Forward => {
                self.horizontal_position += command.magnitude;
                self.depth += self.aim * command.magnitude;
            }
            Up => self.aim -= command.magnitude,
            Down => self.aim += command.magnitude,
        }
        self
    }
}

#[aoc(day2, part1)]
fn part1(input_data: &str) -> Option<u32> {
    let final_position = input_data
        .lines()
        .map(|line| line.trim())
        .filter_map(|input| input.parse().ok())
        .fold(Position::default(), |position, command| {
            position.apply_command(command)
        });
    Some((final_position.horizontal_position * final_position.depth) as u32)
}

#[aoc(day2, part2)]
fn part2(input_data: &str) -> Option<u32> {
    let final_position = input_data
        .lines()
        .map(|line| line.trim())
        .filter_map(|input| input.parse().ok())
        .fold(Position::default(), |position, command| {
            position.apply_command(command)
        });
    Some((final_position.horizontal_position * final_position.depth) as u32)
}

#[test]
fn example() {
    let input_data = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
";
    assert_eq!(part2(&input_data), Some(900));
}
