use std::collections::{HashMap, HashSet};

fn main() {
    println!("Part 1 is {}", part1(include_str!("22_input.txt")));
}

#[derive(Debug)]
enum Direction {
    Move(u32),
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
struct MinMax {
    min: i64,
    max: i64,
}

#[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
struct Position {
    x: i64,
    y: i64,
}

fn part1(input: &'static str) -> i64 {
    let mut all = HashSet::new();
    let mut walls = HashSet::new();
    let mut lines_min_max = HashMap::new();
    let mut columns_min_max = HashMap::new();
    let (map_as_string, directions_as_string) = input.split_once("\n\n").unwrap();

    let mut directions = vec![];

    let mut buffer = String::new();
    for direction_char in directions_as_string.chars() {
        match direction_char {
            'L' => {
                directions.push(Direction::Move(buffer.parse().unwrap()));
                buffer = String::new();
                directions.push(Direction::Left);
            }
            'R' => {
                directions.push(Direction::Move(buffer.parse().unwrap()));
                buffer = String::new();
                directions.push(Direction::Right);
            }
            digit => {
                buffer.push(digit);
            }
        }
    }
    directions.push(Direction::Move(buffer.parse().unwrap()));

    for (y, line) in map_as_string.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '.' || char == '#' {
                all.insert(Position {
                    x: x as i64,
                    y: y as i64,
                });
            }

            if char == '#' {
                walls.insert(Position {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    let max_x = all.iter().map(|position| position.x).max().unwrap();
    let max_y = all.iter().map(|position| position.y).max().unwrap();

    for column in 0..=max_x {
        let min = all
            .iter()
            .filter(|position| position.x == column)
            .map(|position| position.y)
            .min()
            .unwrap();
        let max = all
            .iter()
            .filter(|position| position.x == column)
            .map(|position| position.y)
            .max()
            .unwrap();

        columns_min_max.insert(column, MinMax { min, max });
    }

    for line in 0..=max_y {
        let min = all
            .iter()
            .filter(|position| position.y == line)
            .map(|position| position.x)
            .min()
            .unwrap();
        let max = all
            .iter()
            .filter(|position| position.y == line)
            .map(|position| position.x)
            .max()
            .unwrap();

        lines_min_max.insert(line, MinMax { min, max });
    }

    let mut position = Position {
        x: lines_min_max.get(&0).unwrap().min,
        y: 0,
    };
    let mut orientation = Position { x: 1, y: 0 };

    print_map(&all, &walls, &orientation, &position);

    for direction in directions {
        // print_map(&all, &walls, &orientation, &position);

        match direction {
            Direction::Move(number) => {
                for _ in 0..number {
                    let mut new_position = Position {
                        x: position.x + orientation.x,
                        y: position.y + orientation.y,
                    };
                    if new_position.x > lines_min_max.get(&position.y).unwrap().max {
                        new_position.x = lines_min_max.get(&position.y).unwrap().min;
                    }
                    if new_position.x < lines_min_max.get(&position.y).unwrap().min {
                        new_position.x = lines_min_max.get(&position.y).unwrap().max;
                    }
                    if new_position.y > columns_min_max.get(&position.x).unwrap().max {
                        new_position.y = columns_min_max.get(&position.x).unwrap().min;
                    }
                    if new_position.y < columns_min_max.get(&position.x).unwrap().min {
                        new_position.y = columns_min_max.get(&position.x).unwrap().max;
                    }

                    if !walls.contains(&new_position) {
                        position = new_position;
                    }
                }
            }
            Direction::Left => match orientation {
                Position { x: 1, y: 0 } => orientation = Position { x: 0, y: -1 },
                Position { x: 0, y: 1 } => orientation = Position { x: 1, y: 0 },
                Position { x: -1, y: 0 } => orientation = Position { x: 0, y: 1 },
                Position { x: 0, y: -1 } => orientation = Position { x: -1, y: 0 },
                _ => panic!(),
            },
            Direction::Right => match orientation {
                Position { x: 1, y: 0 } => orientation = Position { x: 0, y: 1 },
                Position { x: 0, y: 1 } => orientation = Position { x: -1, y: 0 },
                Position { x: -1, y: 0 } => orientation = Position { x: 0, y: -1 },
                Position { x: 0, y: -1 } => orientation = Position { x: 1, y: 0 },
                _ => panic!(),
            },
        }
    }

    let orientation_score = match orientation {
        Position { x: 1, y: 0 } => 0,
        Position { x: 0, y: 1 } => 1,
        Position { x: -1, y: 0 } => 2,
        Position { x: 0, y: -1 } => 3,
        _ => panic!(),
    };

    orientation_score + 4 * (position.x + 1) + 1000 * (position.y + 1)
}

fn print_map(
    all: &HashSet<Position>,
    walls: &HashSet<Position>,
    orientation: &Position,
    user_position: &Position,
) {
    for y in 0..210 {
        for x in 0..160 {
            let position = Position { x, y };
            if !all.contains(&position) {
                print!(" ");
                continue;
            }

            if walls.contains(&position) {
                print!("#");
                continue;
            }

            if position == *user_position {
                let char = match orientation {
                    Position { x: 1, y: 0 } => '>',
                    Position { x: 0, y: 1 } => 'v',
                    Position { x: -1, y: 0 } => '<',
                    Position { x: 0, y: -1 } => '^',
                    _ => panic!(),
                };

                print!("{char}");
                continue;
            }

            print!(".");
        }
        println!();
    }
}

#[test]
fn test() {
    assert_eq!(6032, part1(include_str!("./22_test.txt")))
}
