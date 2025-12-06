use std::ops::RangeInclusive;

struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

fn parse(input: &str) -> Input {
    let mut splitted = input.split("\n\n");
    let ranges = splitted.next().expect("two parts");
    let ranges = ranges
        .lines()
        .map(|line| {
            let mut line = line.split("-").map(|x| x.parse().expect("valid number"));
            let a = line.next().expect("two numbers");
            let b = line.next().expect("two numbers");
            a..=b
        })
        .collect();

    let ingredients = splitted.next().expect("two parts");
    let ingredients = ingredients
        .lines()
        .map(|l| l.parse().expect("valid number"))
        .collect();

    Input {
        ranges,
        ingredients,
    }
}

fn part_one(input: &str) -> usize {
    let Input {
        ranges,
        ingredients,
    } = parse(input);
    ingredients
        .iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn part_two(input: &str) -> u64 {
    let Input {
        mut ranges,
        ingredients: _,
    } = parse(input);

    ranges.sort_by_key(|r| *r.start());
    let mut idx = 0;
    ranges.iter().fold(0, |mut acc, range| {
        // Skip the range if already accounted for
        if idx > *range.end() {
            return acc;
        }

        if range.contains(&idx) {
            acc += *range.end() - idx + 1;
        } else {
            acc += *range.end() - *range.start() + 1;
        }

        idx = *range.end() + 1;
        acc
    })
}

aoc25::aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn day5() {
        assert_eq!(part_one(INPUT), 3);
        assert_eq!(part_two(INPUT), 14);
    }
}
