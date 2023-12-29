use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // Parse instructions
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();

    // Parse elements
    let mut elements = HashMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (element, next) = line.split_once('=').unwrap();
        let next = next
            .trim()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap();
        let (left, right) = next.split_once(',').unwrap();

        elements.insert(element.trim(), (left.trim(), right.trim()));
    }

    // Loop until ZZZ
    let mut steps = 0;
    let mut element = "AAA";

    loop {
        if element == "ZZZ" {
            break;
        }

        let (left, right) = elements.get(element).unwrap();

        element = match instructions[steps % instructions.len()] {
            'L' => left,
            'R' => right,
            _ => panic!("Wrong instruction!"),
        };

        steps += 1;
    }

    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "2");
    }

    #[test]
    fn it_also_works() {
        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "6");
    }
}
