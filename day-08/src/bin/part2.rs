use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

// Least common multiple
fn lcm(nums: Vec<usize>) -> usize {
    let mut result = 1;
    for n in nums {
        result = (n * result) / gcd(n, result);
    }
    result
}

// Greatest common divisor
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn part2(input: &str) -> String {
    // Parse instructions
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();

    // Parse elements
    let mut elements = HashMap::new();
    let mut starting_nodes = HashSet::new();

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

        let element = element.trim();
        if element.ends_with('A') {
            starting_nodes.insert(element);
        }

        elements.insert(element, (left.trim(), right.trim()));
    }

    /* NOTE: All starting nodes only has 1 node ending with a Z in their cycle.
    We can therefore find how long it takes for each starting position to find that node,
    and then calculate the least common multiple of all of them
    Tried bruteforcing (running part1 until they all ended with Z) however it took WAY too long*/

    // Finds offset for each starting position
    let offsets: Vec<_> = starting_nodes
        .iter()
        .map(|n| {
            let mut steps = 0;
            let mut current_node = *n;
            // Loop until we find the node ending with Z
            while !current_node.ends_with('Z') {
                let (left, right) = elements.get(current_node).unwrap();
                current_node = match instructions[steps % instructions.len()] {
                    'L' => *left,
                    'R' => *right,
                    _ => panic!("Wrong instruction!"),
                };
                steps += 1;
            }
            steps
        })
        .collect();

    lcm(offsets).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, "6");
    }
}
