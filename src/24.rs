use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

fn main() {
    println!(
        "Part 1 is {}",
        part1(
            "
            #.########################################################################################################################
            #.v^<>^^.<^^<v>v>>>.<^<<v.^>vv>>^<<<v>.v<^v^<v<<^<^^<^<^v><^vv.v^^.>^><vv><^<.<^v<>.v<v..^>><v>.vvv.v>^^<^>.<<^<^<^>v.^^<#
            #.^^<v>><<v><vv<>..<>v<^v><<>><>v.>v.^^<<.v.^<^<vv><v<<v><<^<>^v>v>><>v..v^<^.v><>^>><v<<>v^<<v<v>v.v<v>><<v>^v<v^...<<<>#
            #><vv>>v>v<^>^^^^vv^<v^<<v<>^<^.^^>^<^vv>^v>><<^^<.>^>v<^v<.v.^<^^v.<.>.<^>v^>.v>>^vv^v.>>>v<<<<v.^^v<^^<^><<<<.^>.>vvv>>#
            #<.vv>.><<.<^v.>v^v>><v>><<^vv>^v^>.><v>>><^><v<v^^vv>.v^^<v>vv<>v>^v.>>v^>..v<>^v.^>><.vv><^...<>^^>v>vvv>^<^<><v>v<^^^<#
            #>^.^>vvv>vv>^^vv>v^^v>vv>v^v>^<v>>vvvv^>.v^<v^><^>vvv<>vv<^v<.<^v^>^vv^<><>.^.^>><<^^>^<v^>>>>v><<v^^>^^>>v^v.<^vv^<^v<>#
            #><^>><vv^vvv.v><v<>^v>^<<<^^<><^<<v<>^<<v><<<v>><v<^^<vv<.>^v>^<v>.>v>>^^>^>><>.<<v<^><>>>^.<vv>.^^<.^^>>v.><<v<>><>v^><#
            #>>^.<<v.^v>^^v.>v.vv^<<.v<v<..<<v^^>^v.><>v<^^<<.>v>v>v>^v>.<>v<><<^v><^>><^v>v^v>^vvv>>>..>^>>.><v><>.^^v.><<vv<.v>^^<.#
            #>vv>^^.<>^v<^<<v^>v<^^v><>>>>v^vv>v<^v^><><^.<v^>.<^.v^>><vv>>vvv>^v<<<><vv<v^<<>v>>^v>^^^>^v><^^>^^^<<^^<v<<v<v>^^^<^>>#
            #>.v^<<.vv<^v>^>>v<>v^v^^v^v<^^<<..<<^v<<<<^<v>^>.<>v<v<v<.<.v>vv^>><^.<.v<^<^v><v<v>.v.><<^>>v^.^vv.vv^.>^^>vv<>^..v>^v<#
            #..v.>^><<>..<>.<^<<v>^^^^><<><<vv><^>v>^v<vv>v><vv.^<.<<<<v^v<>^^^vv>.>>>v^v<v<^.<v<<^.v<><<>>v<<<.>>v.^^>^^.>.<.>vv>^>>#
            #>^^vv<<><<v<^.v>>v>^^^v^<v.^><<<<<v^v>v<<^v..^<.vv.^v>vvv>^v<..v<><<^<<vv<>>>>^>^^<>^<^<^^.vv>>^>^>^v^>.<^.<<.>v^>vv<.^<#
            #<v^vv>^^v>vv>^.^>v>v^v>vv.>>.>v^v^>.^v<^.>^<<<v.>^>^^<>vv<.<^^<^v<^><v^v^.^<><v^^v^>.<vv^>v^v<v^><>^<vv>v<><^v<v.<<^.>^>#
            #<>^<^<v><^^.^^>..^^>v>>>>^v<.<>>.<^v^>vvvvv>^<<.><^<v<.v<^><^^>vv<^<^<>v>v<>^^v>>^^>>v<^^<v>v^^^vvv<<<<<>>>v>^v.<^^>^>^<#
            #>.<^v<.<^v.v>^.>>><^<<^^<<.<>><>>>^vvv<v<v<<^^vv<<<^v^v^vv<^v>>^^vv.^>^.v^^^>>v^>.<>.^^>v^^^^><^^>^>>.v<v^<^>^v^>^<<<v<<#
            #>><><.>v^<>^^>.^>^v>^.^<v><><>.<v<^v>>>>v.<>>.^vv>v.vv.><vv^v>^<>>v>vv.v>v>v<><>>>^..^v>^^><..^..^..^vvv>><>v<>v<^<<>v<<#
            #<vv^vvv><>v^v.<^v^^v<.<>v<>.^v^<<^<><.^vv<>^>>.v<<><vv<^^<>>v^.^v><v^^<>v.v<.>^^v^<<<^^<<v<^^^<<<<>^^v>><><<><>v>v<v.>^>#
            #.<.^>^>^>^v..v>^.<v<^<<<^vvv>vv><^^v^<.>>.^<.^^<<>>^><<>>^^.^<<^^><^v<.><>.<<<.vvv>>vv^v<^v<>vv<<^v^^<v.v^<<^>v^>v<><vv<#
            #.vvvv>>^>^<..^>>^>vv^.^>^<<v>^^v>v>^<>.>vv<v^>>>^.^<<^>>^>^>vvv><vv^^<^.^^.<^^v.<v<>^<<<v^>><<.^^^^>^>>>v<<>>v>v<>v<v><.#
            #<<vv.<v>^^v<.>>^>^<>^><.<v^><>>.^<v^v<<.v>>.<>.v>^>^v><><^<><<<v^<><<v^..^v^v>^.><>>>v^.^>vv<<^>v<><.^^v<>vv>v^v<><^>>><#
            #><>^v>v<>><vv>>v<>^<v>v^^>^v^>^^<v>v>^^v><<^v>>vv<^><>^>^^>>v^^<^<v>.^>v^vv^<.v>^v^.v^><><>>v.v^vv^<v>^>^><v>^>^>v>^v..<#
            #>v<^^<^>.<>v<v<.^vv^>>v><^^v<^^>>^>^<^^>>>^<v<^v<>v<^>v>^>v^v>^.<<v^>vv<^<.><^><<^>v><^<v>^<^v>v^>><^vvv^>>v>>vv>v^>^><<#
            #<vvv<><^.>v.^><v<^^<v><^<vvv.^..v^>...v^><<^^><^>^^vv>^v>v^<v<<>^>^^..<^<<v><<>.><<>^vvvv>v^><.^^>>v>>>^>>^<^<^<v^>vvv>>#
            #><<<^^.^<.^.^^<>^.vv^^^<v^<<<vv<<>^<v<^>>v^>>v<<v^^.v^<.vv<v<><<.v><<^>>v><.v><v>v^^v^v<^><vvvvv<^.<<>v^.<>^.^><>^v><>^<#
            #<.<>.>v^><<>^^<<vv>^>^v>>>v>v><^v><v.>>^<>>.<vv<<<v<v^<<vv^<.v^^>vv>>^<^<v<v<<>vv.><v..>>>.>^v<<vv><v^^^vv^^<>^>v<^^>v.>#
            #<>^^<>>^v>v<<>^.>>^^^<<v>^vv^<^<.<<<vv>>v>^^<^v><><>.<>.^><<^^^>^vv><>v.<>v<.<v>>><v.^>^>vv<v>^<v^<<v^v>v..>^^<^^.><>^><#
            ########################################################################################################################.#            
        "
        )
    )
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(input: &'static str) -> usize {
    let mut walls: HashSet<(i64, i64)> = HashSet::new();
    let mut blizzards = HashMap::new();

    for (y, line) in input.trim().lines().map(|line| line.trim()).enumerate() {
        for (x, char) in line.chars().enumerate() {
            let x = x as i64;
            let y = y as i64;

            match char {
                '#' => {
                    walls.insert((x, y));
                }
                '>' => blizzards
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(Direction::Right),
                '<' => blizzards
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(Direction::Left),
                '^' => blizzards
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(Direction::Up),
                'v' => blizzards
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(Direction::Down),
                '.' => {}
                _ => panic!("{char}"),
            }
        }
    }

    let max_x = *walls.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *walls.iter().map(|(_, y)| y).max().unwrap();

    walls.insert((1, -1));
    walls.insert((max_x - 1, max_y + 1));

    let mut blocks_by_minutes = Vec::new();
    let mut blizzards_by_minutes = Vec::new();

    let initial = to_string(max_x, max_y, &blizzards, None);

    loop {
        let mut blocks = walls.clone();
        blocks.extend(blizzards.keys());

        blocks_by_minutes.push(blocks);
        blizzards_by_minutes.push(blizzards.clone());

        let mut new_blizzards = HashMap::new();
        for ((x, y), blizzards) in blizzards {
            for blizzard in blizzards {
                let mut new_position = match blizzard {
                    Direction::Up => (x, y - 1),
                    Direction::Down => (x, y + 1),
                    Direction::Left => (x - 1, y),
                    Direction::Right => (x + 1, y),
                };

                if new_position.0 == 0 {
                    new_position.0 = max_x - 1;
                }
                if new_position.0 == max_x {
                    new_position.0 = 1;
                }
                if new_position.1 == 0 {
                    new_position.1 = max_y - 1;
                }
                if new_position.1 == max_y {
                    new_position.1 = 1;
                }

                new_blizzards
                    .entry(new_position)
                    .or_insert_with(Vec::new)
                    .push(blizzard);
            }
        }

        let result = to_string(max_x, max_y, &new_blizzards, None);
        if result == initial {
            break;
        }

        blizzards = new_blizzards;
    }

    let mut distances: HashMap<(i64, i64, usize), usize> = HashMap::from([((1, 0, 0), 0)]);
    let mut visited: HashSet<(i64, i64, usize)> = HashSet::new();
    let mut distances_to_visit = BinaryHeap::from([Reverse((0, (1, 0, 0)))]);
    loop {
        let to_visit = match distances_to_visit.pop() {
            Some(info) => info,
            None => {
                break;
            }
        };

        let current_position = to_visit.0 .1;
        let current_distance = to_visit.0 .0;

        visited.insert(current_position);

        let new_distance = current_distance + 1;

        for (diff_x, diff_y) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next_position = (current_position.0 + diff_x, current_position.1 + diff_y);

            if !blocks_by_minutes[new_distance % blocks_by_minutes.len()].contains(&next_position) {
                let better = if let Some(existing_distance) = distances.get(&(
                    next_position.0,
                    next_position.1,
                    new_distance % blocks_by_minutes.len(),
                )) {
                    new_distance < *existing_distance
                } else {
                    true
                };

                if better {
                    distances.insert(
                        (
                            next_position.0,
                            next_position.1,
                            new_distance % blocks_by_minutes.len(),
                        ),
                        new_distance,
                    );
                    distances_to_visit.push(Reverse((
                        new_distance,
                        (
                            next_position.0,
                            next_position.1,
                            new_distance % blocks_by_minutes.len(),
                        ),
                    )))
                }
            }
        }
    }

    *distances
        .iter()
        .filter(|((x, y, _), _)| *x == max_x - 1 && *y == max_y)
        .min_by_key(|(_, distance)| *distance)
        .unwrap()
        .1
}

fn to_string(
    max_x: i64,
    max_y: i64,
    blizzards: &HashMap<(i64, i64), Vec<Direction>>,
    position: Option<(i64, i64)>,
) -> String {
    let mut result = String::with_capacity((max_x as usize + 2) * (max_y as usize + 1));
    for y in 0..=max_y {
        let mut line = String::with_capacity(max_x as usize + 1);
        for x in 0..=max_x {
            if let Some(position) = position {
                if position.0 == x && position.1 == y {
                    line.push('E');
                    continue;
                }
            }

            if x == 0 || y == 0 || x == max_x || y == max_y {
                line.push('#');
            } else {
                if let Some(blizzards_at_position) = blizzards.get(&(x, y)) {
                    if blizzards_at_position.len() > 1 {
                        line.push(
                            char::from_digit(blizzards_at_position.len() as u32, 10).unwrap(),
                        );
                    } else {
                        line.push(match blizzards_at_position[0] {
                            Direction::Up => '^',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                            Direction::Right => '>',
                        })
                    }
                } else {
                    line.push('.');
                }
            }
        }
        line.push('\n');
        result.push_str(&line);
    }

    result
}

#[test]
fn test() {
    assert_eq!(
        18,
        part1(
            "
    #.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#
    "
        )
    )
}
