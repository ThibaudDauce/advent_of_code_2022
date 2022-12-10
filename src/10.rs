fn main() {
    println!("Part 1 is {}", part1(input()));
}

fn part1(input: &'static str) -> i32 {
    let cycles = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .flat_map(|line| match line {
            "noop" => vec![0],
            _ => {
                let (operation, value) = line.split_once(' ').unwrap();
                match operation {
                    "addx" => vec![0, value.parse::<i32>().unwrap()],
                    _ => panic!(),
                }
            }
        })
        .enumerate()
        .map(|(cycle, diff)| (cycle + 1, diff));

    let max = 220;
    let to_keep: Vec<_> = (20..=max).step_by(40).collect();
    let mut register = 1;
    let mut result = 0;
    for (cycle, diff) in cycles {
        if cycle > max {
            break;
        }

        if to_keep.contains(&cycle) {
            result += cycle as i32 * register;
        }

        register += diff;
    }

    result
}

#[test]
fn test() {
    assert_eq!(
        13140,
        part1(
            "
    addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
    "
        )
    )
}

fn input() -> &'static str {
    "
    noop
addx 12
addx -5
addx -1
noop
addx 4
noop
addx 1
addx 4
noop
addx 13
addx -8
noop
addx -19
addx 24
addx 1
noop
addx 4
noop
addx 1
addx 5
addx -1
addx -37
addx 16
addx -13
addx 18
addx -11
addx 2
addx 23
noop
addx -18
addx 9
addx -8
addx 2
addx 5
addx 2
addx -21
addx 26
noop
addx -15
addx 20
noop
addx 3
noop
addx -38
addx 3
noop
addx 26
addx -4
addx -19
addx 3
addx 1
addx 5
addx 3
noop
addx 2
addx 3
noop
addx 2
noop
noop
noop
noop
addx 5
noop
noop
noop
addx 3
noop
addx -30
addx -4
addx 1
addx 18
addx -8
addx -4
addx 2
noop
addx 7
noop
noop
noop
noop
addx 5
noop
noop
addx 5
addx -2
addx -20
addx 27
addx -20
addx 25
addx -2
addx -35
noop
noop
addx 4
addx 3
addx -2
addx 5
addx 2
addx -11
addx 1
addx 13
addx 2
addx 5
addx 6
addx -1
addx -2
noop
addx 7
addx -2
addx 6
addx 1
addx -21
addx 22
addx -38
addx 5
addx 3
addx -1
noop
noop
addx 5
addx 1
addx 4
addx 3
addx -2
addx 2
noop
addx 7
addx -1
addx 2
addx 4
addx -10
addx -19
addx 35
addx -1
noop
noop
noop

    "
}
