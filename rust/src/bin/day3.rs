use cached::proc_macro::cached;

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let line: im::Vector<u64> = line.chars().map(|c| (c as u8 - b'0') as u64).collect();
            joltage(line, 1)
        })
        .sum()
}

fn part_two(input: &str) -> u128 {
    input
        .lines()
        .map(|line| {
            let line: im::Vector<u64> = line.chars().map(|c| (c as u8 - b'0') as u64).collect();
            joltage(line, 11) as u128
        })
        .sum()
}

#[cached]
fn joltage(input: im::Vector<u64>, leftover: u32) -> u64 {
    if input.len() <= leftover as usize {
        return input.iter().fold(0, |acc, x| x + acc * 10);
    }

    let next = input.clone().skip(1);
    match leftover {
        0 => u64::max(input[0], joltage(next.clone(), 0)),
        _ => {
            let recursed = joltage(next.clone(), leftover);
            let current = input[0] * u64::pow(10, leftover) + joltage(next, leftover - 1);
            u64::max(recursed, current)
        }
    }
}

aoc25::aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn day3() {
        assert_eq!(part_one(INPUT), 357);
        assert_eq!(part_two(INPUT), 3121910778619);
    }

    #[test]
    fn day3_joltage() {
        assert_eq!(
            joltage(im::vector![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 1),
            98,
        );
        assert_eq!(
            joltage(im::vector![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 11),
            987654321111
        );

        assert_eq!(
            joltage(im::vector![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 1),
            89
        );
        assert_eq!(
            joltage(im::vector![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 11),
            811111111119
        );

        assert_eq!(
            joltage(im::vector![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 1),
            78
        );
        assert_eq!(
            joltage(im::vector![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 11),
            434234234278
        );

        assert_eq!(
            joltage(im::vector![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 1),
            92
        );
        assert_eq!(
            joltage(im::vector![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 11),
            888911112111
        );
    }
}
