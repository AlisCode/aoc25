fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| {
            let ident = l.as_bytes()[0];
            let rest = &l[1..];
            let value: i32 = rest.parse().expect("wrong format");
            match ident {
                b'L' => -value,
                b'R' => value,
                x => panic!("unexpected ident {x}"),
            }
        })
        .collect()
}

fn part_one(input: &str) -> i32 {
    let instructions = parse(input);
    let mut password = 0;
    let mut dial = 50;

    for i in instructions {
        dial += i;
        dial %= 100;
        if dial == 0 {
            password += 1;
        }
    }

    password
}

fn part_two(input: &str) -> i32 {
    let instructions = parse(input);
    let mut password = 0;
    let mut dial = 50;

    for i in instructions {
        let old = dial;
        dial += i;

        let zeros = match i.signum() {
            -1 => (old - 1).div_euclid(100) - (dial - 1).div_euclid(100),
            1 => dial.div_euclid(100),
            _ => unreachable!(),
        };
        password += zeros;
        dial = dial.rem_euclid(100);
    }

    password
}

aoc25::aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn day1() {
        assert_eq!(super::part_one(TEST_INPUT), 3);
        assert_eq!(super::part_two(TEST_INPUT), 6);
    }
}
