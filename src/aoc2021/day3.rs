use super::aoc;

#[aoc(day3, part1)]
fn part1(input_data: &str) -> Option<u32> {
    let counters = input_data
        .lines()
        .map(str::trim)
        .filter_map(|input| u32::from_str_radix(input, 2).ok())
        .fold(
            Vec::with_capacity(u32::BITS as usize),
            |mut counters: Vec<usize>, diagnostic| {
                let bit_size = (u32::BITS - diagnostic.leading_zeros()) as usize;
                counters.resize(counters.len().max(bit_size), 0);
                for position in 0..bit_size {
                    if diagnostic & (1 << position) != 0 {
                        counters[position as usize] += 1;
                    }
                }
                counters
            },
        );
    let half_diagnostic_count = input_data.lines().count() / 2;
    let (gamma_rate, epsilon_rate) = counters.iter().enumerate().fold(
        (0, 0),
        |(gamma_rate, epsilon_rate), (position, counter)| {
            if counter > &half_diagnostic_count {
                (gamma_rate + (1 << position), epsilon_rate)
            } else {
                (gamma_rate, epsilon_rate + (1 << position))
            }
        },
    );
    Some(gamma_rate * epsilon_rate)
}

#[aoc(day3, part2)]
fn part2(input_data: &str) -> Option<u32> {
    let diagnostic_report = input_data
        .lines()
        .map(str::trim)
        .filter_map(|input| u32::from_str_radix(input, 2).ok())
        .collect::<Vec<_>>();
    let counters = diagnostic_report.iter().fold(
        Vec::with_capacity(u32::BITS as usize),
        |mut counters: Vec<usize>, diagnostic| {
            let bit_size = (u32::BITS - diagnostic.leading_zeros()) as usize;
            counters.resize(counters.len().max(bit_size), 0);
            for position in 0..bit_size {
                if diagnostic & (1 << position) != 0 {
                    counters[position as usize] += 1;
                }
            }
            counters
        },
    );
    let half_diagnostic_count = input_data.lines().count() / 2;
    let (gamma_rate, epsilon_rate) = counters.iter().enumerate().fold(
        (0, 0),
        |(mut gamma_rate, mut epsilon_rate), (position, counter)| {
            if counter >= &half_diagnostic_count {
                gamma_rate += 1 << position;
            }
            if counter <= &half_diagnostic_count {
                epsilon_rate += 1 << position;
            }
            (gamma_rate, epsilon_rate)
        },
    );
    let oxygen_generator_rating_criteria =
        |diagnostic: u32| -> u32 { !(diagnostic ^ (gamma_rate | !(gamma_rate ^ epsilon_rate))) };
    let oxygen_generator_rating = diagnostic_report
        .iter()
        .max_by_key(|&&diagnostic| oxygen_generator_rating_criteria(diagnostic))?;
    let co2_scrubber_rating = diagnostic_report
        .iter()
        .max_by_key(|&diagnostic| (diagnostic ^ epsilon_rate).leading_zeros())?;
    println!(
        "gamma_rate:               {:05b} | {:05b} = {:05b}",
        gamma_rate,
        (gamma_rate ^ epsilon_rate),
        gamma_rate | (gamma_rate ^ epsilon_rate)
    );
    println!("epsilon_rate:             {:05b}", epsilon_rate);
    let rate = |diagnostic: u32| {
        let result = oxygen_generator_rating_criteria(diagnostic);
        println!(
            "rate {}  {:05b}:           {:05b}",
            diagnostic, diagnostic, result
        );
    };
    rate(10);
    rate(22);
    rate(23);
    rate(30);
    println!(
        "oxygen_generator_rating:  {:05b} {}",
        oxygen_generator_rating, oxygen_generator_rating
    );
    println!(
        "co2_scrubber_rating:      {:05b} {}",
        co2_scrubber_rating, co2_scrubber_rating
    );
    Some(oxygen_generator_rating * co2_scrubber_rating)
}

#[test]
fn example() {
    let input_data = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";
    assert_eq!(part1(&input_data), Some(198));
    assert_eq!(part2(&input_data), Some(230));
}
