fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
enum Type {
    Number(u32),
    Symbol,
    Dot,
}

fn part1(input: &str) -> String {
    // Parse schematic
    let mut schematic: Vec<Vec<Type>> = vec![];
    for line in input.lines() {
        let line_vec = line
            .chars()
            .map(|c| match c {
                '.' => Type::Dot,
                c => match c.to_digit(10) {
                    Some(n) => Type::Number(n),
                    None => Type::Symbol,
                },
            })
            .collect();
        schematic.push(line_vec);
    }
    // Collect numbers as (line, start, end, value)
    let mut numbers = vec![];
    for (y, line) in schematic.iter().enumerate() {
        let mut start = 0;
        let mut value = 0;

        for (x, t) in line.iter().enumerate() {
            match t {
                Type::Number(i) => {
                    value = value * 10 + i;
                }
                _ => {
                    if value > 0 {
                        numbers.push((y as isize, start, x as isize - 1, value))
                    }
                    start = x as isize + 1;
                    value = 0;
                }
            }
        }
        // Check if we ended on a number
        if value > 0 {
            numbers.push((y as isize, start, line.len() as isize - 1, value))
        }
    }
    // Determine which numbers have a surrounding symbol
    let mut result = 0;
    'number_loop: for (line, start, end, value) in numbers.iter() {
        for i in (line - 1)..=(line + 1) {
            for j in (start - 1)..=(end + 1) {
                // Check bounds
                if 0 <= i
                    && i < schematic.len() as isize
                    && 0 <= j
                    && j < schematic[i as usize].len() as isize
                {
                    // If we're inside bounds check if we have a symbol
                    if let Type::Symbol = schematic[i as usize][j as usize] {
                        // Add the value to the result and continue to the next number
                        result += value;
                        continue 'number_loop;
                    }
                }
            }
        }
    }
    // Return result
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
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
        assert_eq!(result, "4361");
    }
}
