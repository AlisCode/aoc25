use std::collections::HashSet;

use cached::proc_macro::cached;

struct Input {
    start: usize,
    splitters: Vec<HashSet<usize>>,
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let first = lines.next().expect("at least 1 line");
    let start = first
        .char_indices()
        .find_map(|(idx, c)| (c == 'S').then_some(idx))
        .expect("to find the starting position");

    let splitters = lines
        .map(|line| {
            line.char_indices()
                .filter_map(|(idx, c)| (c == '^').then_some(idx))
                .collect()
        })
        .collect();

    Input { start, splitters }
}

fn part_one(input: &str) -> usize {
    let Input { start, splitters } = parse(input);

    let mut tachyons: HashSet<usize> = HashSet::new();
    tachyons.insert(start);

    let mut count = 0;
    for splitters in splitters {
        let mut remove = HashSet::new();
        let new = tachyons
            .intersection(&splitters)
            .flat_map(|x| {
                remove.insert(*x);
                [x - 1, x + 1]
            })
            .collect::<HashSet<usize>>();

        count += remove.len();

        tachyons.extend(new);
        for r in remove.into_iter() {
            tachyons.remove(&r);
        }
    }

    count
}

// With the good ol' immutable collection+memoize trick.
// Never fails.
//
// ... there has to be a smarter way though
fn part_two(input: &str) -> usize {
    let Input { start, splitters } = parse(input);

    // Im not writing a second parser. It's december.
    let splitters_im = splitters
        .into_iter()
        .map(|s| s.into_iter().collect())
        .collect();
    part_two_recurse(start, splitters_im)
}

#[cached]
fn part_two_recurse(idx: usize, mut list_splitters: im::Vector<im::HashSet<usize>>) -> usize {
    let Some(splitters) = list_splitters.pop_front() else {
        return 1;
    };

    if splitters.contains(&idx) {
        return part_two_recurse(idx - 1, list_splitters.clone())
            + part_two_recurse(idx + 1, list_splitters);
    }

    part_two_recurse(idx, list_splitters)
}

aoc25::aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn day7() {
        assert_eq!(part_one(INPUT), 21);
        assert_eq!(part_two(INPUT), 40);
    }
}
