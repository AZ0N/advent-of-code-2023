fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    // List of numbers and their replacement
    // TODO Simplify these replacements. Can remove until parts that may overlap
    let numbers = [
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ];

    let output: u32 = input
        .lines()
        .map(|line| {
            // Preprocess line
            let mut l = line.to_string();

            for (k, v) in &numbers {
                l = l.replace(k, v)
            }

            // Calculate number (same as part1)
            let mut nums = l.chars().filter_map(|c| c.to_digit(10));
            let first = nums.next();
            let last = nums.last();
            match (first, last) {
                (Some(first), Some(last)) => 10 * first + last,
                (Some(first), _) => 11 * first,
                (_, _) => 0,
            }
        })
        .sum();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
