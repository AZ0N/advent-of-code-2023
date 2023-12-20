use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
enum Type {
    Number(u32),
    Gear,
    Other,
}

fn part2(input: &str) -> String {
    // Parse schematic
    let mut schematic: Vec<Vec<Type>> = vec![];
    for line in input.lines() {
        let line_vec = line
            .chars()
            .map(|c| match c {
                '*' => Type::Gear,
                c => match c.to_digit(10) {
                    Some(n) => Type::Number(n),
                    None => Type::Other,
                },
            })
            .collect();
        schematic.push(line_vec);
    }
    // Collect numbers and gears
    let mut numbers = BTreeMap::new();
    let mut gears = vec![];
    for (y, line) in schematic.iter().enumerate() {
        let mut start = 0;
        let mut value = 0;
        for (x, t) in line.iter().enumerate() {
            match t {
                Type::Number(i) => {
                    value = value * 10 + i;
                }
                _ => {
                    if let Type::Gear = t {
                        gears.push((y, x));
                    }
                    if value > 0 {
                        for k in start..=(x.saturating_sub(1)) {
                            numbers.insert((y, k), (y, start, value));
                        }
                    }
                    start = x + 1;
                    value = 0;
                }
            }
        }
        // Check if we ended on a number
        if value > 0 {
            for k in start..=line.len().saturating_sub(1) {
                numbers.insert((y, k), (y, start, value));
            }
        }
    }
    // Check gears and calculate result
    let mut result = 0;
    for (y, x) in gears {
        let mut numbers_around = vec![];

        for i in (y.saturating_sub(1))..=(y + 1).min(schematic.len() - 1) {
            for j in (x.saturating_sub(1))..=(x + 1).min(schematic[i].len()) {
                // Check bounds
                if let Some(x) = numbers.get(&(i, j)) {
                    if !(numbers_around.contains(x)) {
                        numbers_around.push(*x);
                    }
                }
            }
        }
        // Check if there are 2 numbers around and calculate gear ratio
        if numbers_around.len() == 2 {
            let mut gear_ratio = 1;
            for (_, _, v) in numbers_around {
                gear_ratio *= v;
            }
            result += gear_ratio;
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "467835");
    }
}
