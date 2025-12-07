#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

struct Problem<'a> {
    numbers: Vec<&'a str>,
    op: Op,
}

fn parse<'a>(input: &'a str) -> Vec<Problem<'a>> {
    let mut numbers = Vec::new();
    let mut ops = Vec::new();

    for line in input.lines() {
        if line.contains('*') {
            // last line
            for value in line.split_whitespace() {
                match value {
                    "+" => ops.push(Op::Add),
                    "*" => ops.push(Op::Mul),
                    _ => panic!("unknown val"),
                }
            }
            continue;
        }

        let mut line_numbers = Vec::new();
        for value in line.split_whitespace() {
            line_numbers.push(value);
        }
        numbers.push(line_numbers);
    }

    // sanity check
    let len = numbers[0].len();
    if !numbers.iter().all(|n| n.len() == len) {
        panic!("all numbers list should be of the same length");
    }
    if !ops.len() == len {
        panic!("ops should be of the same length");
    }

    (0..len)
        .map(|idx| {
            let problem_numbers = numbers.iter().map(|n| n[idx]).collect();
            Problem {
                numbers: problem_numbers,
                op: ops[idx],
            }
        })
        .collect()
}

fn part_one(input: &str) -> u64 {
    let problems = parse(input);
    problems
        .iter()
        .map(|p| match p.op {
            Op::Add => p
                .numbers
                .iter()
                .map(|x| x.parse::<u64>().expect("valid number"))
                .sum::<u64>(),
            Op::Mul => p
                .numbers
                .iter()
                .map(|x| x.parse::<u64>().expect("valid number"))
                .product(),
        })
        .sum::<u64>()
}

fn part_two(input: &str) -> u64 {
    let mut lines: Vec<&str> = input.lines().collect();

    let sep = lines
        .iter()
        .map(|l| {
            l.char_indices()
                .filter_map(|(idx, c)| match c {
                    ' ' => Some(idx),
                    _ => None,
                })
                .collect::<im::HashSet<usize>>()
        })
        .reduce(|acc, h| acc.intersection(h))
        .expect("to get the list of separators");
    let mut sep: Vec<usize> = sep.into_iter().collect();
    sep.push(0);
    sep.sort();

    let ops = lines.pop().expect("to have ops");
    let ops: Vec<Op> = ops
        .split_whitespace()
        .map(|c| match c {
            "+" => Op::Add,
            "*" => Op::Mul,
            c => panic!("Unknown op {c}"),
        })
        .collect();

    let numbers = lines
        .into_iter()
        .map(|line| {
            sep.windows(2)
                .map(|range| {
                    let start = match range[0] {
                        0 => 0,
                        x => x + 1,
                    };
                    &line[start..range[1]]
                })
                .chain(Some(&line[*sep.last().expect("last elem") + 1..]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (0..ops.len())
        .map(|idx| {
            let longest = numbers.iter().map(|n| n[idx].len()).max().expect("max len");
            let nbs = (0..longest).map(|num_idx| {
                let s = numbers
                    .iter()
                    .filter_map(|n| match n[idx].chars().nth(num_idx) {
                        Some(c) if c != ' ' => Some(c),
                        _ => None,
                    })
                    .collect::<String>();
                s.parse::<u64>().expect("valid number")
            });
            match ops[idx] {
                Op::Add => nbs.sum::<u64>(),
                Op::Mul => nbs.product(),
            }
        })
        .sum()
}

aoc25::aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT_A: &str = "123 328  51 64 ";
    const INPUT_B: &str = " 45 64  387 23 ";
    const INPUT_C: &str = "  6 98  215 314";
    const INPUT_D: &str = "*   +   *   +  ";

    // Spaces are actually important to the problem.
    fn input() -> String {
        [INPUT_A, INPUT_B, INPUT_C, INPUT_D].join("\n")
    }

    #[test]
    fn day6() {
        assert_eq!(part_one(&input()), 4277556);
        assert_eq!(part_two(&input()), 3263827);
    }
}
