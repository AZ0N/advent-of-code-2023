fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let output: u32 = input
        .lines()
        .map(|line| {
            let mut nums = line.chars().filter_map(|c| c.to_digit(10));
            let first = nums.next();
            let last = nums.last();
            match (first, last) {
                (Some(f), Some(l)) => 10 * f + l,
                (Some(f), _) => 11 * f,
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
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142");
    }
}
