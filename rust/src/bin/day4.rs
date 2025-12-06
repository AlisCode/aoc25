use std::collections::HashSet;

use cached::proc_macro::cached;

struct Map {
    pub data: HashSet<(i32, i32)>,
}

fn parse(input: &str) -> Map {
    let data = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '@' => Some((x as i32, y as i32)),
                '.' => None,
                c => panic!("Unexpected char {c}"),
            })
        })
        .collect::<HashSet<(i32, i32)>>();
    Map { data }
}

// memo, we night need these multiple times
// this helps with perf on part two, worsens part one
#[cached]
fn around(x: i32, y: i32) -> HashSet<(i32, i32)> {
    (y - 1..=y + 1)
        .flat_map(|yy| (x - 1..=x + 1).map(move |xx| (xx, yy)))
        .filter(|(xx, yy)| *xx != x || *yy != y)
        .collect()
}

fn paper_rolls(data: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    data.iter()
        .filter(|(x, y)| {
            let neighbors = around(*x, *y);
            let inter = neighbors.intersection(data);
            inter.count() < 4
        })
        .cloned()
        .collect()
}

fn part_one(input: &str) -> usize {
    let Map { data } = parse(input);

    let rolls = paper_rolls(&data);
    rolls.len()
}

// stupid brute-force
fn part_two(input: &str) -> usize {
    let Map { mut data, .. } = parse(input);

    let mut count = 0;
    loop {
        let remove: HashSet<(i32, i32)> = paper_rolls(&data);
        for r in &remove {
            data.remove(r);
        }

        count += remove.len();

        if remove.is_empty() {
            break;
        }
    }

    count
}

aoc25::aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn day4() {
        assert_eq!(part_one(INPUT), 13);
        assert_eq!(part_two(INPUT), 43);
    }
}
