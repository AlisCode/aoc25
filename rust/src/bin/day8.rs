use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug)]
struct Coords {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coords {
    pub fn dist(&self, other: &Coords) -> i64 {
        let x = (self.x - other.x) * (self.x - other.x);
        let y = (self.y - other.y) * (self.y - other.y);
        let z = (self.z - other.z) * (self.z - other.z);
        i64::isqrt(x + y + z)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Distance {
    distance: i64,
    a: usize,
    b: usize,
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Ordering")
    }
}

struct Top {
    list: BinaryHeap<Distance>,
    maxlen: usize,
}

impl Top {
    fn new(maxlen: usize) -> Self {
        Top {
            list: BinaryHeap::new(), // Can't use with_capacity, usize::MAX is too big
            maxlen,
        }
    }

    fn insert(&mut self, val: Distance) {
        if self.list.len() < self.maxlen {
            self.list.push(val);
            return;
        }

        let peeked = self.list.peek().expect("some values in the heap");
        if *peeked > val {
            let _ = self.list.pop();
            self.list.push(val);
        }
    }

    fn into_sorted_vec(self) -> Vec<Distance> {
        self.list.into_sorted_vec()
    }
}

fn parse(input: &str) -> Vec<Coords> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|x| x.parse().expect("valid number"));
            let x = coords.next().expect("3 coords");
            let y = coords.next().expect("3 coords");
            let z = coords.next().expect("3 coords");
            Coords { x, y, z }
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    solve_part_one(input, 1000)
}

fn solve_part_one(input: &str, maxlen: usize) -> usize {
    let coords = parse(input);
    let distances = shortest_connections(&coords, maxlen);

    let mut circuits = Circuits::default();
    for distance in &distances {
        circuits.add(distance);
    }
    circuits.top3()
}

fn part_two(input: &str) -> i64 {
    let coords = parse(input);
    let distances = shortest_connections(&coords, usize::MAX);

    let mut circuits = Circuits::default();
    for distance in &distances {
        circuits.add(distance);

        if circuits.one_big_circuit(coords.len()) {
            return coords[distance.a].x * coords[distance.b].x;
        }
    }

    panic!("It was never one big circuit")
}

fn shortest_connections(coords: &[Coords], maxlen: usize) -> Vec<Distance> {
    let len = coords.len();
    let mut top = Top::new(maxlen);
    for a in 0..len {
        for b in a + 1..len {
            let dist = coords[a].dist(&coords[b]);
            top.insert(Distance {
                distance: dist,
                a,
                b,
            });
        }
    }

    top.into_sorted_vec()
}

#[derive(Default)]
struct Circuits {
    circuit_id: usize,
    circuits: HashMap<usize, HashSet<usize>>,
}

impl Circuits {
    pub fn add(&mut self, distance: &Distance) {
        let Distance { distance: _, a, b } = distance;
        let circuit_a = self
            .circuits
            .iter()
            .find_map(|(idx, c)| c.contains(a).then_some(*idx));
        let circuit_b = self
            .circuits
            .iter()
            .find_map(|(idx, c)| c.contains(b).then_some(*idx));
        match (circuit_a, circuit_b) {
            (None, None) => {
                let mut set = HashSet::default();
                set.insert(*a);
                set.insert(*b);
                self.circuits.insert(self.circuit_id, set);
                self.circuit_id += 1;
            }
            (Some(idx), None) | (None, Some(idx)) => {
                let set = self.circuits.get_mut(&idx).expect("to find an entry");
                set.insert(*a);
                set.insert(*b);
            }
            (Some(idx_a), Some(idx_b)) => {
                if idx_a == idx_b {
                    return;
                }
                let mut set = self
                    .circuits
                    .remove(&idx_a)
                    .expect("to find an entry for A");
                let set_b = self
                    .circuits
                    .remove(&idx_b)
                    .expect("to find an entry for A");
                set.extend(set_b);
                self.circuits.insert(idx_a, set);
            }
        }
    }

    fn one_big_circuit(&self, coords_len: usize) -> bool {
        if self.circuits.len() != 1 {
            return false;
        }
        let circuit = self.circuits.values().next().expect("exactly 1 entry");
        circuit.len() == coords_len
    }

    fn top3(&self) -> usize {
        let mut circuit_length: Vec<_> = self
            .circuits
            .values()
            .map(|circuit| Reverse(circuit.len()))
            .collect();
        circuit_length.sort();

        circuit_length.iter().map(|Reverse(x)| x).take(3).product()
    }
}

aoc25::aoc!(part_one, part_two);

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn day8() {
        assert_eq!(solve_part_one(INPUT, 10), 40);
        assert_eq!(part_two(INPUT), 25272);
    }
}
