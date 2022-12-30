use std::collections::{HashMap, HashSet};

fn main() {
    let (size, max_round) = compute(input());
    println!("Part 1 is {size}");
    println!("Part 2 is {max_round}");
}

fn compute(input: &'static str) -> (i64, usize) {
    let mut elves = HashSet::new();

    for (y, line) in input.trim().lines().map(|line| line.trim()).enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    let arounds = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    //   123
    //   8E4
    //   765

    let mut all_rules = [
        [[1 as usize, 2, 3], [7, 6, 5], [1, 8, 7], [3, 4, 5]],
        [[7, 6, 5], [1, 8, 7], [3, 4, 5], [1, 2, 3]],
        [[1, 8, 7], [3, 4, 5], [1, 2, 3], [7, 6, 5]],
        [[3, 4, 5], [1, 2, 3], [7, 6, 5], [1, 8, 7]],
    ]
    .iter()
    .cycle();

    let mut movement_rules = vec![None; 9];
    movement_rules[2] = Some((0, -1));
    movement_rules[4] = Some((1, 0));
    movement_rules[6] = Some((0, 1));
    movement_rules[8] = Some((-1, 0));

    // println!("\n\n== Initial position ==\n");
    // print_map(&elves);

    let mut round = 1;
    let mut size = None;
    let max_round = loop {
        let mut movements = HashMap::with_capacity(elves.len());

        let rules = all_rules.next().unwrap();

        for (x, y) in &elves {
            let mut occupied = Vec::with_capacity(arounds.len());
            let mut has_at_least_of_elf = false;
            for (around_x, around_y) in &arounds {
                let has_elf = elves.contains(&(x + around_x, y + around_y));
                if has_elf {
                    has_at_least_of_elf = true;
                }
                occupied.push(has_elf);
            }

            if has_at_least_of_elf {
                'rule_loop: for rule in rules {
                    for position in rule {
                        if occupied[*position - 1] {
                            continue 'rule_loop;
                        }
                    }

                    let (diff_x, diff_y) = movement_rules[rule[1]].unwrap();
                    let new_position = (x + diff_x, y + diff_y);
                    movements
                        .entry(new_position)
                        .or_insert_with(Vec::new)
                        .push((*x, *y));
                    break 'rule_loop;
                }
            }
        }

        if movements.is_empty() {
            break round;
        }

        for (new_position, old_positions) in &movements {
            if old_positions.len() == 1 {
                elves.remove(&old_positions[0]);

                assert!(!elves.contains(new_position));
                elves.insert(*new_position);
            }
        }

        // println!("\n\n== After Round {} ==\n", round);
        // print_map(&elves);

        if round == 10 {
            let min_x = elves.iter().map(|(x, _)| x).min().unwrap();
            let max_x = elves.iter().map(|(x, _)| x).max().unwrap();

            let min_y = elves.iter().map(|(_, y)| y).min().unwrap();
            let max_y = elves.iter().map(|(_, y)| y).max().unwrap();

            size = Some((max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i64);
        }

        round += 1;
    };

    (size.unwrap(), max_round)
}

#[allow(dead_code)]
fn print_map(elves: &HashSet<(i64, i64)>) {
    let min_x = elves.iter().map(|(x, _)| x).min().unwrap();
    let max_x = elves.iter().map(|(x, _)| x).max().unwrap();

    let min_y = elves.iter().map(|(_, y)| y).min().unwrap();
    let max_y = elves.iter().map(|(_, y)| y).max().unwrap();

    for y in min_y - 1..=max_y + 1 {
        for x in min_x - 1..=max_x + 1 {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

#[test]
fn test() {
    assert_eq!(
        (110, 20),
        compute(
            "
            ....#..
            ..###.#
            #...#.#
            .#...##
            #.###..
            ##.#.##
            .#..#..
    "
        )
    );
}

fn input() -> &'static str {
    "
    #.....###.....#.###....#.#..#...###..#..#....#.#.......#.##.###.####.#...#
    ##.#..#.#.#.#..##.##......####.##.##...#...##..#.....####..#.#..###...#..#
    .#.###...##.#.###.#..##.#..#.###..#.###..#..##.##.########....#...#...##..
    ##.#...#.##..#...############.#..#..##.#.#.#.####....#.##....#...##.####..
    ##.#.###....#.##.##.#.####....####.###.#..#...#####.#..###.#.###.....#.#..
    .#..#####.#...###..#..#..###..#.#...#.###.###.#.##.#.....#.#.##.##.#.....#
    #.#...##..#.###.##.##.##........#.#.##....#..##...#..#####..##..#....#.##.
    ###....###..#..#.....#..####...###.#.#....#.#.#.#..##.###.#..#..######....
    #..#..#...#.#.#.#....#.#.##.#.#.#.#######..#.##...##..####.###.##.#.###.#.
    ###.##...#...##..#...#.##.##.##.#.#.#.#..#.#.#...###....#.#.##.....###....
    ....##...##.#.##.#..#...#..#...#.##.#...##..#...#.##...##..#.#.##..#.#....
    ##.....#.##....##...###.#.#####...####.###....#######.#.....##.#.#..#.##..
    .......#####..#.###.#...###..##.##..#.#.###...#..#..##..##.#..#.#..#..##..
    #.#..###.#.#.#..#####....###.#..#.#.###...##....#.#.####.##...#....#.#.#..
    ..#.....##.#.#..###.#...######.##....#####.###...####.#####.##..#...###.#.
    .#.####.......##.###.#..#####.#######...##.#.###..#......##..#...#.#.#.#..
    ##.###.....#.##..#.#.#.#.#..##....####.#...#.###...#.####..#.#......#..#.#
    ####.......##....######.#.#.#.#..#.##.###.....#...#.#..#.#.###..#.###..##.
    ##..###..###...##...#.#..##.#..#.#.#.#...#..##..#.#..#....#...##...##.###.
    #....#.....####..##.#........#.#..#...#...##...##.#..#..#.#.#.#..#......##
    #.#.#..#..##.####......#..#####.#.##..##..##...####.###..#####...#.#..##.#
    ####.#.##.....#..####.##.####.#####...#####.#....##....#..#..#.###.#.##.#.
    ..#.##.##.##.#.#...#.###..#.##.######..#..#...#.###..###.###.#.#.#..#.#.#.
    ..##..##..#..#####...##.###...###.###########.#..#######..#.#.#....#######
    #.###.#.##..######.###..##...#..##.#.###..##..#..#.#.#..#.##.#.#..#.#.####
    #....#.##...##..#.#.####.......#.#.###.#......###......#..##...#..#.####..
    ##.####..######.####....#.###.##..#.#.##.#.#..##.#..##.#.##.#.##.#####..#.
    #######.####.#.##.########..#..###.###.###...##......#..##.#.#####...#...#
    #..##.#.#..######.##.##..#....#..##.#.#####.##..#.##.##..#..###.#.##..##.#
    ..#.#.#..##...####....##.#.#..####..#.#####.###.#..##.....#..##..##.#.#..#
    #.###..####....####..###.#.####.#.##....#.##.###.##.###########..#..###.##
    ..##.##.##.#.####......####...##..#....#####...#.#....#####..#.#####..##.#
    #..##...#...###.#..#.##.##.#.#.#....#..##.###.###.##........#..####.##...#
    ...#####.####..###........#.##...##....#...#..#...##..###..#####...######.
    ..#..##..#.......##.#....###..######.##.####.##..##.....##.###.#.#####.#.#
    ..#.#...#.......##.##.###..#.#.#...#########..###...#....#...#.###.#..#.##
    #.#..#..######..#........##..###.##..#.##.#.#..#.#....##.#.#..#.#...#.#.#.
    #....#.#.#####.###......#..#..##.##...#......##..#.###..##.####.######.#..
    #..#.....#..####.#...##.####..######.#.#.###..##.....##.#.##.######.#.####
    ....###.#######..#.##..########.....#.#..##..#.#..#.##....#.#..#..#..##.#.
    #..#.##.#.#.#.##.#.#.....#..#..#...#..##..##.#..#.##.#.#.#...##.##.#...#.#
    .#.######.#..##.#.##.....##.#.##.#.#....######..##..##.#...#...#.#.###..##
    .#.#..#..#..#..##.#.####.#.####.#....#.#.###.###.#..###..##.#...#.###.###.
    ##.#.##..###.####.#.#.###.#......#####.###.######.#..######.#...#.#.####..
    ###.####..#.###....#.....#.#.##.#.##.#####...##.#...#####.#.###.#..##.....
    ##..#.#..#.####.#.####...###.#.##..#####.#..###...#####.####..#...#.##.#.#
    ..##.#####..##.#####..#..#..#..#...##..#.###..##.#....##...#...#.#.##...##
    ##...#.##..#.##...#.#.####..#.#...####..#.##.#.#...#####.##..#######.##...
    .#..####.##...#.#...##..#..###.#####.#.#.#.#..##..##.###.....#...#..####..
    .##..#...#.###.####.#.#...#...#####..#..#.#.##...#...##..#.#..#.#.#.###..#
    ###.##.####.##....#.#..#.##.##..###.####..##..#.####.....#.#.###..#.######
    #.##.#.##.#..#..#.#......##.#.##.#..#.#.#.....###.....#......###....#..###
    .#..#..#....#..#.#...#..#..##......##...###.###.##...#...#.....#...###.###
    #..##..#..##.#....###...#......#.######..#.###..#.#....#......#.##..#.####
    .#...##.#..#####..##...#####.#....#######....#.....###....#..###.##..#..#.
    .###.#.##..#..#..##..#....#...#.##.##.##.#..#...##..#..##...##..######.###
    #....#.#####.###..#.#.......##..##...##..#.###..#####.#..#..##.#..#....##.
    ##..#......#######...#.##.###..#.....##..#.####.###.##.#..####.#.#####.##.
    ..#....###.#........#..######.#...##.......##.#.###.##.#...#.##....##.#...
    ###.....#..##.####.#..###..#..#####.##.##.#######.#.####.####......#.##.##
    ..#...#...#######.#.####....#.####..#..#..##.#.#.###..#.#####.##.#.....#.#
    ###.#.#..#.####.##...#.#####.##.####..#.#.###.#..#...#..#########.###.#.##
    ..######.########..###.#.#.###.#.#.#.#.####.#...#..##...##.#####..##...#.#
    ...#.#...##.#....####.###..#..#..#..##..###.##..#.#....#..#...######.##.##
    #####.##..#..##..#####.#..#..#..###.##..##...###....#.###.#.#.##...###..##
    ..#.##.#....####..#.#..###.#.#..##...#..##....####.............#.#.#..#.##
    .###...#.#....#..#.######....###.....#..#...##..##.#....##...#.###....###.
    .#.####..#.#.###..###.#..###.#..#..#######....###.......#....###.#...#.###
    ###.#.###.#..#.#..###.#.....#.##..#.#.####..###...#...#####...####...##...
    .#..##..#.##..#..##..######.###.##.....##..#..#.##...####.#...#####..###.#
    ..##...######.#.#...#.#.#####.##.##...##.##....#.#####...###....#.#.#.#..#
    ##..#.#.#....##.####.#.##.##.#.#.##......#...##..##...##.#####...#..#..##.
    ...####.#..###....#.###.#.#.........#....####..#........#.###..####.##...#
    .##.##.#####...##.....##.#..#####..##.....#....##......#.#..###..###..##..
    "
}
