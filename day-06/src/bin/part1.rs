fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    // Parse race times
    let times: Vec<usize> = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();

    // Parse race distances
    let distances: Vec<usize> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();

    let output = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| {
            (1..*t)
                .map(|i| if i * (*t - i) > *d { 1 } else { 0 })
                .sum::<usize>()
        })
        .product::<usize>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "288");
    }
}
