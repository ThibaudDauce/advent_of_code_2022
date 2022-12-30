use part2::*;

fn main() {
    println!("Part 1 is {}", part1::part1(include_str!("22_input.txt")));
    println!(
        "Part 2 is {}",
        part2::part2(
            include_str!("22_input.txt"),
            50,
            &[
                (
                    (1, 0),
                    Mappings {
                        x: Mapping::X,
                        y: Mapping::Y,
                        z: Mapping::Static(0),
                        offset: V3 { x: 0, y: 0, z: 0 },
                    }
                ),
                (
                    (2, 0),
                    Mappings {
                        x: Mapping::Static(50),
                        y: Mapping::Y,
                        z: Mapping::X,
                        offset: V3 { x: 0, y: 0, z: 1 },
                    }
                ),
                (
                    (1, 1),
                    Mappings {
                        x: Mapping::X,
                        y: Mapping::Static(50),
                        z: Mapping::Y,
                        offset: V3 { x: 0, y: 0, z: 1 },
                    }
                ),
                (
                    (1, 2),
                    Mappings {
                        x: Mapping::X,
                        y: Mapping::MinusY,
                        z: Mapping::Static(51),
                        offset: V3 { x: 0, y: 0, z: 0 },
                    }
                ),
                (
                    (0, 2),
                    Mappings {
                        x: Mapping::Static(-1),
                        y: Mapping::MinusY,
                        z: Mapping::X,
                        offset: V3 { x: 0, y: 0, z: 1 },
                    }
                ),
                (
                    (0, 3),
                    Mappings {
                        x: Mapping::Y,
                        y: Mapping::Static(-1),
                        z: Mapping::X,
                        offset: V3 { x: 0, y: 0, z: 1 },
                    }
                ),
            ],
        )
    );
}

mod part1 {
    use std::collections::{HashMap, HashSet};

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

    pub fn part1(input: &'static str) -> i64 {
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

        // print_map(&all, &walls, &orientation, &position);

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

    #[allow(dead_code)]
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
}

mod part2 {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug)]
    enum Rotation {
        Left,
        Right,
    }

    #[derive(Debug)]
    enum Direction {
        Move(u32),
        Rotate(Rotation),
    }

    #[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
    struct MinMax {
        min: i64,
        max: i64,
    }

    #[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
    pub struct V2 {
        x: i64,
        y: i64,
    }
    #[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
    pub struct V3 {
        pub x: i64,
        pub y: i64,
        pub z: i64,
    }

    #[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
    #[allow(dead_code)]
    pub enum Mapping {
        Static(i64),
        X,
        MinusX,
        Y,
        MinusY,
    }

    #[derive(Clone, Debug)]
    pub struct Mappings {
        pub x: Mapping,
        pub y: Mapping,
        pub z: Mapping,
        pub offset: V3,
    }

    fn convert_2d_to_3d(position_in_2d: V2, square: i64, face_mappings: &Mappings) -> V3 {
        V3 {
            x: convert_one_coordinate_2d_to_3d(
                position_in_2d,
                square,
                face_mappings.x,
                face_mappings.offset.x,
            ),
            y: convert_one_coordinate_2d_to_3d(
                position_in_2d,
                square,
                face_mappings.y,
                face_mappings.offset.y,
            ),
            z: convert_one_coordinate_2d_to_3d(
                position_in_2d,
                square,
                face_mappings.z,
                face_mappings.offset.z,
            ),
        }
    }

    fn convert_one_coordinate_2d_to_3d(
        position_in_2d: V2,
        square: i64,
        mapping: Mapping,
        offset: i64,
    ) -> i64 {
        let coordinate = match mapping {
            Mapping::Static(new) => new,
            Mapping::X => position_in_2d.x,
            Mapping::MinusX => (position_in_2d.x * -1) + (square - 1),
            Mapping::Y => position_in_2d.y,
            Mapping::MinusY => (position_in_2d.y * -1) + (square - 1),
        };

        coordinate + offset
    }

    #[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
    enum Space {
        Empty,
        Wall,
    }

    #[derive(Debug)]
    struct Rollback {
        // face_static: Axis,
        face_mappings: Mappings,
        face_coordinates: V2,
        original_coordinates: V2,
    }

    pub fn part2(input: &'static str, square: i64, mappings: &[((i64, i64), Mappings)]) -> i64 {
        let mut all = HashSet::new();
        let mut walls = HashSet::new();
        let (map_as_string, directions_as_string) = input.split_once("\n\n").unwrap();

        let mut directions = vec![];

        let mut buffer = String::new();
        for direction_char in directions_as_string.chars() {
            match direction_char {
                'L' => {
                    directions.push(Direction::Move(buffer.parse().unwrap()));
                    buffer = String::new();
                    directions.push(Direction::Rotate(Rotation::Left));
                }
                'R' => {
                    directions.push(Direction::Move(buffer.parse().unwrap()));
                    buffer = String::new();
                    directions.push(Direction::Rotate(Rotation::Right));
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
                    all.insert(V2 {
                        x: x as i64,
                        y: y as i64,
                    });
                }

                if char == '#' {
                    walls.insert(V2 {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }

        let mut rollback_3d: HashMap<V3, Rollback> = HashMap::new();
        let mut cube: HashMap<V3, Space> = HashMap::new();

        for ((face_x, face_y), face_mappings) in mappings {
            for x in 0..square {
                for y in 0..square {
                    let position_in_2d_coordinates = V2 {
                        x: x as i64 + face_x * square,
                        y: y as i64 + face_y * square,
                    };

                    assert!(all.contains(&position_in_2d_coordinates));

                    let position_in_3d_coordinates =
                        convert_2d_to_3d(V2 { x, y }, square, &face_mappings);

                    let space = if walls.contains(&position_in_2d_coordinates) {
                        Space::Wall
                    } else {
                        Space::Empty
                    };

                    // let face_static = if matches!(face_mappings.x, Mapping::Static(_)) {
                    //     Axis::X
                    // } else if matches!(face_mappings.y, Mapping::Static(_)) {
                    //     Axis::Y
                    // } else {
                    //     assert!(matches!(face_mappings.z, Mapping::Static(_)));
                    //     Axis::Z
                    // };

                    cube.insert(position_in_3d_coordinates, space);
                    rollback_3d.insert(
                        position_in_3d_coordinates,
                        Rollback {
                            face_mappings: face_mappings.clone(),
                            face_coordinates: V2 {
                                x: *face_x,
                                y: *face_y,
                            },
                            original_coordinates: position_in_2d_coordinates,
                        },
                    );
                }
            }
        }

        let mut position = V3 { x: 0, y: 0, z: 0 };
        let mut orientation = V3 { x: 1, y: 0, z: 0 };

        for direction in directions {
            dbg!(&direction, position, orientation);
            print_map(
                &all,
                &walls,
                &get_orientation_2d(&rollback_3d, position, orientation),
                &rollback_3d.get(&position).unwrap().original_coordinates,
            );

            match direction {
                Direction::Move(number) => {
                    'move_loop: for _ in 0..number {
                        let new_position = V3 {
                            x: position.x + orientation.x,
                            y: position.y + orientation.y,
                            z: position.z + orientation.z,
                        };

                        if let Some(space) = cube.get(&new_position) {
                            if *space == Space::Empty {
                                position = new_position;
                            } else {
                                break 'move_loop;
                            }
                        } else {
                            let mut arounds = Vec::with_capacity(6);

                            for diff_x in [-1, 1] {
                                arounds.push(V3 {
                                    x: new_position.x + diff_x,
                                    y: new_position.y,
                                    z: new_position.z,
                                });
                            }
                            for diff_y in [-1, 1] {
                                arounds.push(V3 {
                                    x: new_position.x,
                                    y: new_position.y + diff_y,
                                    z: new_position.z,
                                });
                            }
                            for diff_z in [-1, 1] {
                                arounds.push(V3 {
                                    x: new_position.x,
                                    y: new_position.y,
                                    z: new_position.z + diff_z,
                                });
                            }

                            for around in &arounds {
                                if *around == position {
                                    continue;
                                }

                                if let Some(space) = cube.get(&around) {
                                    if *space == Space::Empty {
                                        position = *around;
                                        orientation = V3 {
                                            x: around.x - new_position.x,
                                            y: around.y - new_position.y,
                                            z: around.z - new_position.z,
                                        };
                                        continue 'move_loop;
                                    } else {
                                        break 'move_loop;
                                    }
                                }
                            }

                            dbg!(new_position, arounds);
                            panic!("Not found something around ");
                        }
                    }
                }
                Direction::Rotate(rotation) => {
                    let rollback = rollback_3d.get(&position).unwrap();
                    let original_coordinates = rollback.original_coordinates;

                    let mut orientation_2d =
                        get_orientation_2d(&rollback_3d, position, orientation);

                    match rotation {
                        Rotation::Left => match orientation_2d {
                            V2 { x: 1, y: 0 } => orientation_2d = V2 { x: 0, y: -1 },
                            V2 { x: 0, y: 1 } => orientation_2d = V2 { x: 1, y: 0 },
                            V2 { x: -1, y: 0 } => orientation_2d = V2 { x: 0, y: 1 },
                            V2 { x: 0, y: -1 } => orientation_2d = V2 { x: -1, y: 0 },
                            _ => panic!(),
                        },
                        Rotation::Right => match orientation_2d {
                            V2 { x: 1, y: 0 } => orientation_2d = V2 { x: 0, y: 1 },
                            V2 { x: 0, y: 1 } => orientation_2d = V2 { x: -1, y: 0 },
                            V2 { x: -1, y: 0 } => orientation_2d = V2 { x: 0, y: -1 },
                            V2 { x: 0, y: -1 } => orientation_2d = V2 { x: 1, y: 0 },
                            _ => panic!(),
                        },
                    }

                    let new_position_2d_with_new_orientation = V2 {
                        x: original_coordinates.x + orientation_2d.x,
                        y: original_coordinates.y + orientation_2d.y,
                    };

                    let new_position_3d_with_new_orientation = convert_2d_to_3d(
                        V2 {
                            x: new_position_2d_with_new_orientation.x
                                - rollback.face_coordinates.x * square,
                            y: new_position_2d_with_new_orientation.y
                                - rollback.face_coordinates.y * square,
                        },
                        square,
                        &rollback.face_mappings,
                    );

                    orientation = V3 {
                        x: new_position_3d_with_new_orientation.x - position.x,
                        y: new_position_3d_with_new_orientation.y - position.y,
                        z: new_position_3d_with_new_orientation.z - position.z,
                    };
                }
            }
        }

        let rollback = rollback_3d.get(&position).unwrap();

        let orientation_score = match get_orientation_2d(&rollback_3d, position, orientation) {
            V2 { x: 1, y: 0 } => 0,
            V2 { x: 0, y: 1 } => 1,
            V2 { x: -1, y: 0 } => 2,
            V2 { x: 0, y: -1 } => 3,
            _ => panic!(),
        };

        orientation_score
            + 4 * (rollback.original_coordinates.x + 1)
            + 1000 * (rollback.original_coordinates.y + 1)
    }

    fn print_map(all: &HashSet<V2>, walls: &HashSet<V2>, orientation: &V2, user_position: &V2) {
        for y in 0..20 {
            for x in 0..20 {
                let position = V2 { x, y };
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
                        V2 { x: 1, y: 0 } => '>',
                        V2 { x: 0, y: 1 } => 'v',
                        V2 { x: -1, y: 0 } => '<',
                        V2 { x: 0, y: -1 } => '^',
                        _ => '@',
                    };

                    print!("{char}");
                    continue;
                }

                print!(".");
            }
            println!();
        }
    }

    fn get_orientation_2d(
        rollback_3d: &HashMap<V3, Rollback>,
        position: V3,
        orientation: V3,
    ) -> V2 {
        let original_coordinates = rollback_3d.get(&position).unwrap().original_coordinates;

        let next_position = V3 {
            x: position.x + orientation.x,
            y: position.y + orientation.y,
            z: position.z + orientation.z,
        };

        if let Some(next_rollback) = rollback_3d.get(&next_position) {
            V2 {
                x: next_rollback.original_coordinates.x - original_coordinates.x,
                y: next_rollback.original_coordinates.y - original_coordinates.y,
            }
        } else {
            let previous_position = V3 {
                x: position.x - orientation.x,
                y: position.y - orientation.y,
                z: position.z - orientation.z,
            };

            let previous_position_2d = rollback_3d
                .get(&previous_position)
                .unwrap()
                .original_coordinates;

            V2 {
                x: original_coordinates.x - previous_position_2d.x,
                y: original_coordinates.y - previous_position_2d.y,
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(6032, part1::part1(include_str!("./22_test.txt")));
    assert_eq!(
        5031,
        part2::part2(
            include_str!("./22_test.txt"),
            4,
            &[
                (
                    (2, 0),
                    Mappings {
                        x: Mapping::X,
                        y: Mapping::Y,
                        z: Mapping::Static(0),
                        offset: V3 { x: 0, y: 0, z: 0 },
                    }
                ),
                (
                    (2, 1),
                    Mappings {
                        x: Mapping::X,
                        y: Mapping::Static(4),
                        z: Mapping::Y,
                        offset: V3 { x: 0, y: 0, z: 1 },
                    }
                ),
                (
                    (1, 1),
                    Mappings {
                        x: Mapping::Static(-1),
                        y: Mapping::X,
                        z: Mapping::Y,
                        offset: V3 { x: 0, y: 0, z: 1 },
                    }
                ),
                (
                    (0, 1),
                    Mappings {
                        x: Mapping::MinusX,
                        y: Mapping::Static(-1),
                        z: Mapping::Y,
                        offset: V3 { x: 0, y: 0, z: 1 },
                    }
                ),
                (
                    (2, 2),
                    Mappings {
                        x: Mapping::X,
                        y: Mapping::MinusY,
                        z: Mapping::Static(5),
                        offset: V3 { x: 0, y: 0, z: 0 },
                    }
                ),
                (
                    (3, 2),
                    Mappings {
                        x: Mapping::Static(4),
                        y: Mapping::MinusY,
                        z: Mapping::MinusX,
                        offset: V3 { x: 0, y: 0, z: 1 },
                    }
                ),
            ],
        )
    );
}
