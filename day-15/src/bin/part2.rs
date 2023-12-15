fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn hash_step(prev: i32, c: char) -> i32 {
    ((prev + c as i32) * 17) % 256
}

fn hash(s: &str) -> i32 {
    s.chars().fold(0, |acc,c| hash_step(acc, c))
}

fn part2(input: &str) -> String {
    // Initialize boxes
    let mut boxes: Vec<Vec<(&str, i32)>> = Vec::new();
    boxes.resize(256, Vec::new());

    // Loop through steps
    for step in input.split(',') {
        // Parse step type
        if step.contains('=') {
            // = step
            let (label, focal) = step.split_once('=').unwrap();
            let focal: i32 = focal.parse().unwrap();
            let hash = hash(label);
            // Add to box
            let b = boxes.get_mut(hash as usize).unwrap();
            // Check if there is already a lens with the label and get it's index
            let index = b.iter().position(|(l,_)| *l == label);
            // Add the lens if it's not already there, else change the lens
            match index {
                None => b.push((label, focal)),
                Some(index) => b[index] = (label, focal)
            }
        }
        else if step.contains('-') {
            // - step
            let label = step.strip_suffix('-').unwrap();
            let hash = hash(label);

            // Remove from box
            let b = boxes.get_mut(hash as usize).unwrap();
            b.retain(|(l,_)| *l != label);
        }
    };

    // Map each box to it's total focusing power, and sum it up
    let output: i32 = boxes.iter().enumerate().map(|(i, b)| {
        // Map each lens to it's focusing power, and sum it up
        b.iter().enumerate().map(|(j,(_, f))| ((1+i as i32)*(1+j as i32)*f)).sum::<i32>()
    }).sum();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        );
        assert_eq!(result, "145");
    }
}