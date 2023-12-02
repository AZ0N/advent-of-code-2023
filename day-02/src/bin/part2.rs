fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let output: u32 = input
        .lines()
        .map(|line| {
            let line = line
                .split(':')
                .last()
                .expect("Should have a semicolon in each line");

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for set in line.split(';') {
                for cube in set.split(',') {
                    let mut cube = cube.trim().split(' ');
                    let num = match cube.next().expect("Should have a number").parse::<u32>() {
                        Ok(n) => n,
                        _ => panic!("Expected a number!"),
                    };
                    let color = cube.next().expect("Should have a color");

                    if color.contains("red") && num > red {
                        red = num;
                    }
                    else if color.contains("green") && num > green {
                        green = num;
                    }
                    else if color.contains("blue") && num > blue {
                        blue = num;
                    }
                }
            }
            red * green * blue
        }).sum();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286");
    }
}
