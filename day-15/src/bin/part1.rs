fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn hash_step(prev: i32, c: char) -> i32 {
    ((prev + c as i32) * 17) % 256
}

fn hash(s: &str) -> i32 {
    s.chars().fold(0, |acc,c| hash_step(acc, c))
}

fn part1(input: &str) -> String {
    let output: i32 = input.split(',').map(|s| hash(s)).sum();
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        );
        assert_eq!(result, "1320");
    }

    #[test]
    fn hash_step_works() {
        let result = hash_step(0, 'H');
        assert_eq!(result, 200);

        let result = hash_step(result, 'A');
        assert_eq!(result, 153);

        let result = hash_step(result, 'S');
        assert_eq!(result, 172);

        let result = hash_step(result, 'H');
        assert_eq!(result, 52);
    }

    #[test]
    fn hash_works() {
        let result = hash("HASH");
        assert_eq!(result, 52);
    }
}