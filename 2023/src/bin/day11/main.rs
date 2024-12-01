use aoc_2023::aoc::*;

fn input() -> &'static str {
    include_str!("input.txt")
}

fn part1(data: &str) -> u64 {
    let (map, galaxies) = parse::galaxy(data);
    let number = galaxies.len();
    let mut distances = vec![];
    for a in 0..number {
        for b in (a + 1)..number {
            let distance = get_distance(&map, galaxies[a], galaxies[b], 2);
            distances.push(distance);
        }
    }
    distances.iter().sum()
}

fn get_distance(map: &Vec<Vec<char>>, a: (usize, usize), b: (usize, usize), expansion: u32) -> u64 {
    let mut distance = 0;
    let startx = a.0.min(b.0);
    let endx = a.0.max(b.0);
    let starty = a.1.min(b.1);
    let endy = a.1.max(b.1);
    for x in startx + 1..=endx {
        let c = map[starty].get(x).unwrap();
        if *c == '.' || *c == '#' {
            distance += 1;
        } else {
            distance += expansion;
        }
    }

    for y in starty + 1..=endy {
        let c = map[y].get(endx).unwrap();
        if *c == '.' || *c == '#' {
            distance += 1;
        } else {
            distance += expansion;
        }
    }
    distance.into()
}

fn part2(data: &str, expansion: u32) -> u64 {
    let (map, galaxies) = parse::galaxy(data);
    let number = galaxies.len();
    let mut distances = vec![];
    for a in 0..number {
        for b in (a + 1)..number {
            let distance = get_distance(&map, galaxies[a], galaxies[b], expansion);
            distances.push(distance);
        }
    }
    distances.iter().sum()
}

fn main() {
    println!("Day 11 of {}", version());
    let val = part1(input());
    println!("Part 1 answer is {}", val);
    let val = part2(input(), 1_000_000);
    println!("Part 2 answer is {}", val);
}

mod parse {
    fn transpose(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
        (0..input[0].len())
            .map(|i| input.iter().map(|inner| inner[i]).collect::<Vec<_>>())
            .collect()
    }

    pub fn galaxy(i: &str) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
        let mut lines = vec![];
        for line in i.lines() {
            if line.chars().all(|e| e == '.') {
                lines.push(line.chars().map(|_| '+').collect::<String>());
            } else {
                lines.push(line.to_string())
            }
        }
        let grid = lines
            .iter()
            .map(|e| e.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let transposed = transpose(grid);
        let mut lines = vec![];
        for line in transposed {
            if line.iter().all(|e| *e == '.' || *e == '+') {
                lines.push(line.iter().map(|_| '+').collect::<Vec<char>>());
            } else {
                lines.push(line.clone())
            }
        }
        let mut result = vec![];
        for (x, line) in lines.iter().enumerate() {
            for (y, c) in line.iter().enumerate() {
                if *c == '#' {
                    result.push((x, y));
                }
            }
        }

        let map = transpose(lines);

        (map, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("sample.txt")
    }

    #[test]
    fn test_parse_sample() {
        let (_map, galaxies) = parse::galaxy(sample());
        assert_eq!(9, galaxies.len());
    }

    #[test]
    fn test_sample() {
        assert_eq!(374, part1(sample()));
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(8410, part2(sample(), 100));
    }
}
