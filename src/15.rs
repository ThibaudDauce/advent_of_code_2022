use std::collections::HashSet;

fn main() {
    println!("Part 1 is {}", part1(input(), 2_000_000));
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    position: Position,
    beacon: Position,
    distance: i32,
}

fn part1(input: &'static str, target_y: i32) -> usize {
    let mut sensors = vec![];
    for line in input.trim().lines().map(|line| line.trim()) {
        let line = line.strip_prefix("Sensor at ").unwrap();
        let (position_as_string, beacon_as_string) =
            line.split_once(": closest beacon is at ").unwrap();

        let (x_as_string, y_as_string) = position_as_string.split_once(", ").unwrap();
        let x = x_as_string.strip_prefix("x=").unwrap().parse().unwrap();
        let y = y_as_string.strip_prefix("y=").unwrap().parse().unwrap();

        let position = Position { x, y };

        let (x_as_string, y_as_string) = beacon_as_string.split_once(", ").unwrap();
        let x = x_as_string.strip_prefix("x=").unwrap().parse().unwrap();
        let y = y_as_string.strip_prefix("y=").unwrap().parse().unwrap();

        let beacon = Position { x, y };

        sensors.push(Sensor {
            position,
            beacon,
            distance: (position.x - beacon.x).abs() + (position.y - beacon.y).abs(),
        })
    }

    let mut ranges = vec![];
    for sensor in &sensors {
        let distance_y = (sensor.position.y - target_y).abs();

        if distance_y > sensor.distance {
            continue;
        }

        ranges.push(
            (sensor.position.x - (sensor.distance - distance_y))
                ..=(sensor.position.x + (sensor.distance - distance_y)),
        )
    }

    ranges.sort_by_key(|range| *range.start());

    let mut sum = 0;
    let mut last = i32::MIN;
    for range in ranges {
        for i in range {
            if i > last {
                sum += 1;
                last = i;
            }
        }
    }

    let number_of_beacons_on_target_y = sensors
        .iter()
        .filter(|sensor| sensor.beacon.y == target_y)
        .map(|sensor| sensor.beacon.x)
        .collect::<HashSet<i32>>()
        .len();

    sum - number_of_beacons_on_target_y
}

#[test]
fn test() {
    assert_eq!(
        26,
        part1(
            "
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
            
    ",
            10,
        )
    );
}

fn input() -> &'static str {
    "
    Sensor at x=2692921, y=2988627: closest beacon is at x=2453611, y=3029623
Sensor at x=1557973, y=1620482: closest beacon is at x=1908435, y=2403457
Sensor at x=278431, y=3878878: closest beacon is at x=-1050422, y=3218536
Sensor at x=1432037, y=3317707: closest beacon is at x=2453611, y=3029623
Sensor at x=3191434, y=3564121: closest beacon is at x=3420256, y=2939344
Sensor at x=3080887, y=2781756: closest beacon is at x=3420256, y=2939344
Sensor at x=3543287, y=3060807: closest beacon is at x=3420256, y=2939344
Sensor at x=2476158, y=3949016: closest beacon is at x=2453611, y=3029623
Sensor at x=3999769, y=3985671: closest beacon is at x=3420256, y=2939344
Sensor at x=2435331, y=2200565: closest beacon is at x=1908435, y=2403457
Sensor at x=3970047, y=2036397: closest beacon is at x=3691788, y=1874066
Sensor at x=2232167, y=2750817: closest beacon is at x=2453611, y=3029623
Sensor at x=157988, y=333826: closest beacon is at x=-1236383, y=477990
Sensor at x=1035254, y=2261267: closest beacon is at x=1908435, y=2403457
Sensor at x=1154009, y=888885: closest beacon is at x=1070922, y=-543463
Sensor at x=2704724, y=257848: closest beacon is at x=3428489, y=-741777
Sensor at x=3672526, y=2651153: closest beacon is at x=3420256, y=2939344
Sensor at x=2030614, y=2603134: closest beacon is at x=1908435, y=2403457
Sensor at x=2550448, y=2781018: closest beacon is at x=2453611, y=3029623
Sensor at x=3162759, y=2196461: closest beacon is at x=3691788, y=1874066
Sensor at x=463834, y=1709480: closest beacon is at x=-208427, y=2000000
Sensor at x=217427, y=2725325: closest beacon is at x=-208427, y=2000000
Sensor at x=3903198, y=945190: closest beacon is at x=3691788, y=1874066

    "
}
