use crate::utils::read_lines;

type Range = (i32, i32);

#[derive(Debug)]
struct Sensor {
    pub sx: i32,
    pub sy: i32,

    pub bx: i32,
    pub by: i32,
}

impl Sensor {
    fn beacon_distance(&self) -> i32 {
        (self.sx - self.bx).abs() + (self.sy - self.by).abs()
    }

    fn no_beacon_range(&self, y: i32) -> Option<Range> {
        let beacon_distance = self.beacon_distance();
        let center = beacon_distance - (self.sy - y).abs();
        if center < 0 {
            return None;
        }
        Some((self.sx - center, self.sx + center))
    }
}

fn intersect(r1: &Range, r2: &Range) -> Option<Range> {
    if r1.0 > r2.0 {
        return intersect(r2, r1);
    }
    assert!(
        r1.0 <= r2.0,
        "First range has to start earlier {:?}, {:?}",
        r1,
        r2
    );
    if r1.1 < r2.0 {
        return None;
    }
    Some((std::cmp::max(r1.0, r2.0), std::cmp::min(r1.1, r2.1)))
}

fn sum(r1: &Range, r2: &Range) -> Vec<Range> {
    if intersect(r1, r2) == None {
        return vec![*r1, *r2];
    }
    vec![(r1.0, std::cmp::max(r1.1, r2.1))]
}

fn sum_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort();
    let mut unioned = vec![ranges[0]];
    for i in 1..ranges.len() {
        let last = unioned.pop().unwrap();
        unioned.extend(sum(&last, &ranges[i]));
    }
    unioned
}

fn count_occupied(ranges: &[Range]) -> i32 {
    ranges.iter().map(|r| r.1 - r.0).sum()
}

pub fn run() {
    let lines = read_lines("in/day15.in").unwrap();

    let mut sensors = vec![];

    for line in lines {
        let l = line.unwrap();
        let sensor = scan!(&l;
             ("Sensor at x=", let sx: i32, ", y=", let sy: i32, ": closest beacon is at x=", let bx: i32, ", y=", let by: i32) => Sensor{sx, sy, bx, by}
        ).unwrap();
        sensors.push(sensor);
    }

    let y = 2000000;
    let beacon_ranges: Vec<Range> = sensors
        .iter()
        .filter_map(|s| s.no_beacon_range(y))
        .collect();
    let where_not_part_1 = count_occupied(&sum_ranges(beacon_ranges));
    println!("Day 15, part 1: {}", where_not_part_1);
    for y in 0..=4_000_000 {
        let beacon_ranges: Vec<Range> = sum_ranges(
            sensors
                .iter()
                .filter_map(|s| s.no_beacon_range(y))
                .collect(),
        );
        let intersected: Vec<Range> = beacon_ranges
            .iter()
            .filter_map(|range| intersect(range, &(0, 4_000_000)))
            .collect();
        if intersected.iter().any(|r| r == &(0, 4_000_000)) {
            for x in 0..=4_000_000 {
                if intersected.iter().any(|r| r.0 <= x && x <= r.1) {
                    println!("Day 15, part 2: {}", x as i64 * 4_000_000 + y as i64);
                    return;
                }
            }
        }
    }
}
