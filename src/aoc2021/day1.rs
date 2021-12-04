use super::aoc;

#[aoc(day1, part1)]
fn part1(input_data: &str) -> Option<u32> {
    let depths = input_data
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .collect::<Vec<_>>();
    Some(
        depths
            .as_slice()
            .windows(2)
            .filter(|window| match window {
                [a, b] if a < b => true,
                _ => false,
            })
            .count() as u32,
    )
}

#[aoc(day1, part2)]
fn part2(input_data: &str) -> Option<u32> {
    let depths = input_data
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let depths_sum = depths
        .as_slice()
        .windows(3)
        .filter_map(|window| match window {
            [a, b, c] => Some(a + b + c),
            _ => None,
        })
        .collect::<Vec<_>>();
    Some(
        depths_sum
            .as_slice()
            .windows(2)
            .filter(|window| match window {
                [a, b] if a < b => true,
                _ => false,
            })
            .count() as u32,
    )
}

#[test]
fn example() {
    let input_data = "199
200
208
210
200
207
240
269
260
263
";
    assert_eq!(part1(&input_data), Some(7));
    assert_eq!(part2(&input_data), Some(5));
}
