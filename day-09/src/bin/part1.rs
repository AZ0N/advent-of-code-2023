fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // Parse input
    let mut lines: Vec<Vec<Vec<isize>>> = input
        .lines()
        .map(|line| vec![line.split(' ').filter_map(|x| x.parse().ok()).collect()])
        .collect();

    // Create history for each line
    for line in lines.iter_mut() {
        while !line.last().unwrap().iter().all(|x| *x == 0) {
            let mut new_line: Vec<isize> = vec![];
            for x in line.last().unwrap().windows(2) {
                new_line.push(x[1] - x[0]);
            }
            line.push(new_line);
        }
    }

    // Compute output by sum of last element in each sequence
    let output: isize = lines
        .iter()
        .map(|line| line.iter().map(|l| l.last().unwrap_or(&0)).sum::<isize>())
        .sum();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, "114");
    }
}
