fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut seeds = vec![];

    let mut map_index = 0;
    let mut maps = vec![vec![]; 7];

    // Parse input
    for line in input.lines() {
        // Parse seeds
        if line.starts_with("seeds:") {
            seeds = line
                .strip_prefix("seeds: ")
                .unwrap()
                .split(' ')
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            continue;
        }
        // Ignore newlines and first map-title (to account for map_index)
        if line.is_empty() || line == "seed-to-soil map:" {
            continue;
        }
        // Increment map_index when a new map starts
        if line.contains(':') {
            map_index += 1;
            continue;
        }
        // Else, parse map entry
        let mut iter = line.split(' ').map(|n| n.parse::<u32>().unwrap());
        let (dest, source, length) = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );
        maps[map_index].push((dest, source, length))
    }
    // Convert seeds using each map
    for map in maps {
        seeds = seeds.iter().map(|s| convert_num(*s, &map)).collect();
    }
    // Get minimum location
    seeds.iter().min().unwrap().to_string()
}

fn convert_num(n: u32, map: &Vec<(u32,u32,u32)>) -> u32 {
    for (dest, source, length) in map {
        if source <= &n && n < source + length {
            return dest + (n - source);
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, "35");
    }
}
