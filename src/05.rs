fn main() {
    println!("Part1 is {}", part1(input()));
}

fn part1(input: &'static str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks_as_vec: Vec<&str> = stacks.lines().filter(|line| !line.is_empty()).collect();

    let number_of_columns: usize = stacks_as_vec
        .pop()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut columns: Vec<Vec<char>> = vec![vec![]; number_of_columns];

    stacks_as_vec.reverse();

    for stack_line in stacks_as_vec {
        let chars: Vec<char> = stack_line.chars().collect();
        for column in 0..number_of_columns {
            let index = column * 4 + 1;
            if chars[index] != ' ' {
                columns[column].push(chars[index]);
            }
        }
    }

    for instruction in instructions.trim().lines() {
        let (a, b) = instruction.trim().split_once(" from ").unwrap();
        let (_, quantity) = a.split_once(' ').unwrap();
        let quantity = quantity.parse::<usize>().unwrap();

        let (start, end) = b.split_once(" to ").unwrap();
        let start = start.parse::<usize>().unwrap();
        let end = end.parse::<usize>().unwrap();

        for _ in 0..quantity {
            let one_char = columns[start - 1].pop().unwrap();
            columns[end - 1].push(one_char)
        }
    }

    let mut result = String::with_capacity(number_of_columns);
    for column in 0..number_of_columns {
        result.push(columns[column].pop().unwrap());
    }

    result
}

#[test]
fn test() {
    assert_eq!(
        "CMZ",
        &part1(
            "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    "
        )
    )
}

fn input() -> &'static str {
    "
    [G]         [P]         [M]    
    [V]     [M] [W] [S]     [Q]    
    [N]     [N] [G] [H]     [T] [F]
    [J]     [W] [V] [Q] [W] [F] [P]
[C] [H]     [T] [T] [G] [B] [Z] [B]
[S] [W] [S] [L] [F] [B] [P] [C] [H]
[G] [M] [Q] [S] [Z] [T] [J] [D] [S]
[B] [T] [M] [B] [J] [C] [T] [G] [N]
 1   2   3   4   5   6   7   8   9 

move 2 from 4 to 2
move 6 from 9 to 7
move 4 from 7 to 2
move 2 from 4 to 1
move 2 from 6 to 7
move 1 from 3 to 8
move 4 from 7 to 1
move 2 from 3 to 2
move 3 from 8 to 5
move 3 from 1 to 4
move 12 from 2 to 5
move 2 from 6 to 8
move 12 from 5 to 8
move 3 from 7 to 9
move 18 from 8 to 9
move 2 from 8 to 6
move 3 from 2 to 3
move 14 from 9 to 4
move 1 from 1 to 3
move 7 from 9 to 3
move 1 from 2 to 1
move 8 from 4 to 5
move 5 from 6 to 3
move 2 from 7 to 9
move 3 from 4 to 9
move 4 from 9 to 6
move 4 from 6 to 1
move 8 from 4 to 6
move 10 from 1 to 2
move 13 from 3 to 2
move 17 from 5 to 9
move 2 from 5 to 1
move 9 from 9 to 7
move 1 from 3 to 6
move 2 from 1 to 8
move 11 from 2 to 4
move 5 from 6 to 8
move 1 from 6 to 3
move 1 from 1 to 4
move 3 from 8 to 6
move 3 from 2 to 8
move 9 from 7 to 9
move 4 from 4 to 7
move 1 from 9 to 5
move 15 from 9 to 7
move 7 from 8 to 3
move 1 from 5 to 6
move 2 from 6 to 9
move 8 from 2 to 6
move 3 from 4 to 3
move 1 from 2 to 5
move 4 from 9 to 3
move 1 from 3 to 4
move 13 from 6 to 2
move 1 from 5 to 1
move 4 from 4 to 9
move 6 from 3 to 2
move 11 from 2 to 7
move 6 from 3 to 4
move 3 from 3 to 2
move 1 from 3 to 4
move 1 from 1 to 3
move 3 from 9 to 2
move 1 from 3 to 1
move 4 from 7 to 1
move 1 from 9 to 5
move 5 from 1 to 4
move 11 from 2 to 4
move 1 from 5 to 3
move 1 from 2 to 3
move 12 from 4 to 2
move 2 from 7 to 2
move 7 from 4 to 3
move 5 from 4 to 1
move 7 from 7 to 6
move 4 from 1 to 8
move 1 from 8 to 5
move 8 from 3 to 2
move 4 from 7 to 4
move 13 from 7 to 1
move 2 from 8 to 6
move 5 from 4 to 9
move 1 from 3 to 6
move 1 from 5 to 8
move 1 from 2 to 9
move 4 from 2 to 6
move 2 from 8 to 6
move 10 from 1 to 3
move 4 from 9 to 4
move 2 from 1 to 3
move 5 from 2 to 9
move 4 from 9 to 2
move 1 from 1 to 2
move 13 from 2 to 4
move 15 from 4 to 5
move 3 from 6 to 8
move 8 from 3 to 8
move 1 from 4 to 2
move 14 from 5 to 1
move 1 from 5 to 4
move 1 from 4 to 2
move 8 from 6 to 7
move 3 from 6 to 2
move 2 from 9 to 1
move 8 from 8 to 7
move 9 from 1 to 5
move 7 from 5 to 3
move 14 from 7 to 9
move 2 from 2 to 3
move 7 from 2 to 1
move 1 from 6 to 1
move 4 from 9 to 2
move 8 from 3 to 6
move 2 from 4 to 3
move 4 from 3 to 5
move 5 from 5 to 7
move 2 from 6 to 9
move 6 from 6 to 2
move 4 from 2 to 3
move 1 from 6 to 2
move 2 from 7 to 8
move 13 from 9 to 5
move 2 from 7 to 1
move 14 from 1 to 5
move 15 from 5 to 7
move 3 from 8 to 7
move 5 from 3 to 5
move 6 from 5 to 7
move 4 from 1 to 7
move 1 from 2 to 5
move 3 from 2 to 8
move 11 from 5 to 2
move 10 from 7 to 1
move 1 from 3 to 4
move 10 from 2 to 9
move 1 from 5 to 8
move 6 from 7 to 3
move 1 from 4 to 6
move 2 from 3 to 8
move 1 from 2 to 1
move 4 from 3 to 9
move 3 from 1 to 6
move 2 from 7 to 1
move 1 from 5 to 6
move 1 from 3 to 8
move 4 from 1 to 4
move 5 from 2 to 9
move 3 from 1 to 4
move 18 from 9 to 7
move 4 from 8 to 4
move 3 from 1 to 2
move 1 from 9 to 7
move 1 from 4 to 7
move 1 from 6 to 2
move 1 from 2 to 5
move 25 from 7 to 3
move 7 from 4 to 2
move 8 from 7 to 9
move 4 from 8 to 6
move 1 from 8 to 5
move 4 from 6 to 5
move 2 from 9 to 5
move 3 from 5 to 8
move 4 from 6 to 4
move 12 from 3 to 5
move 11 from 3 to 2
move 13 from 5 to 8
move 4 from 9 to 6
move 7 from 4 to 9
move 2 from 6 to 2
move 12 from 2 to 7
move 1 from 6 to 3
move 1 from 5 to 6
move 2 from 5 to 3
move 15 from 8 to 6
move 4 from 6 to 7
move 1 from 5 to 1
move 10 from 2 to 8
move 8 from 8 to 3
move 8 from 6 to 8
move 2 from 7 to 6
move 9 from 9 to 7
move 8 from 8 to 9
move 1 from 1 to 3
move 1 from 2 to 7
move 7 from 3 to 1
move 3 from 8 to 5
move 3 from 1 to 6
move 7 from 9 to 2
move 2 from 3 to 7
move 5 from 7 to 9
move 17 from 7 to 5
move 2 from 7 to 6
move 10 from 6 to 3
move 1 from 1 to 3
move 6 from 9 to 3
move 1 from 2 to 9
move 2 from 7 to 9
move 2 from 9 to 7
move 1 from 5 to 8
move 1 from 8 to 5
move 6 from 2 to 5
move 1 from 6 to 1
move 5 from 3 to 5
move 1 from 6 to 8
move 1 from 7 to 9
move 2 from 9 to 3
move 15 from 5 to 2
move 2 from 1 to 8
move 2 from 3 to 7
move 2 from 8 to 3
move 3 from 5 to 9
move 1 from 8 to 6
move 1 from 9 to 6
move 3 from 7 to 6
move 17 from 3 to 4
move 1 from 1 to 2
move 6 from 2 to 9
move 16 from 4 to 1
move 4 from 6 to 8
move 9 from 5 to 6
move 8 from 6 to 2
move 2 from 9 to 5
move 2 from 3 to 5
move 1 from 6 to 2
move 1 from 4 to 8
move 14 from 1 to 3
move 8 from 5 to 3
move 20 from 3 to 1
move 1 from 8 to 2
move 1 from 9 to 6
move 1 from 6 to 7
move 1 from 7 to 3
move 22 from 1 to 2
move 3 from 3 to 6
move 27 from 2 to 8
move 2 from 2 to 8
move 2 from 6 to 9
move 2 from 9 to 4
move 2 from 4 to 8
move 1 from 1 to 3
move 14 from 8 to 5
move 1 from 3 to 9
move 3 from 9 to 2
move 5 from 2 to 8
move 10 from 2 to 9
move 1 from 6 to 7
move 1 from 7 to 5
move 7 from 5 to 2
move 2 from 9 to 2
move 1 from 6 to 2
move 2 from 9 to 5
move 3 from 5 to 6
move 6 from 5 to 3
move 1 from 5 to 6
move 4 from 3 to 9
move 2 from 9 to 8
move 3 from 9 to 5
move 23 from 8 to 1
move 2 from 6 to 1
move 1 from 5 to 7
move 2 from 3 to 5
move 2 from 9 to 5
move 4 from 9 to 7
move 2 from 9 to 4
move 1 from 5 to 4
move 5 from 8 to 5
move 2 from 6 to 2
move 3 from 7 to 3
move 1 from 3 to 4
move 3 from 2 to 8
move 4 from 1 to 6
move 2 from 6 to 3
move 4 from 1 to 2
move 3 from 8 to 1
move 13 from 2 to 5
move 4 from 3 to 2
move 14 from 5 to 7
move 5 from 2 to 7
move 18 from 7 to 9
move 4 from 4 to 7
move 2 from 5 to 4
move 17 from 9 to 5
move 1 from 9 to 1
move 1 from 7 to 2
move 5 from 7 to 2
move 18 from 1 to 4
move 1 from 7 to 3
move 1 from 3 to 6
move 2 from 1 to 3
move 1 from 6 to 5
move 2 from 6 to 8
move 1 from 8 to 9
move 1 from 8 to 3
move 13 from 4 to 5
move 1 from 1 to 6
move 3 from 2 to 4
move 1 from 6 to 1
move 3 from 2 to 9
move 3 from 3 to 1
move 5 from 4 to 5
move 30 from 5 to 3
move 1 from 4 to 6
move 1 from 9 to 8
move 1 from 9 to 6
move 21 from 3 to 7
move 3 from 1 to 6
move 1 from 1 to 4
move 1 from 9 to 6
move 1 from 8 to 2
move 1 from 3 to 6
move 1 from 9 to 3
move 5 from 4 to 8
move 1 from 2 to 4
move 9 from 5 to 7
move 2 from 5 to 9
move 2 from 8 to 2
move 2 from 6 to 3
move 1 from 4 to 1
move 4 from 3 to 8
move 2 from 9 to 2
move 4 from 2 to 6
move 1 from 1 to 4
move 2 from 6 to 9
move 2 from 5 to 4
move 1 from 3 to 1
move 1 from 1 to 3
move 2 from 9 to 1
move 5 from 3 to 5
move 1 from 1 to 8
move 4 from 6 to 4
move 5 from 5 to 6
move 18 from 7 to 5
move 1 from 3 to 4
move 12 from 7 to 5
move 15 from 5 to 6
move 1 from 5 to 8
move 1 from 3 to 7
move 1 from 1 to 2
move 1 from 2 to 4
move 1 from 7 to 9
move 2 from 8 to 2
move 1 from 2 to 4
move 4 from 4 to 2
move 1 from 2 to 1
move 1 from 9 to 8
move 4 from 6 to 4
move 3 from 2 to 6
move 1 from 2 to 6
move 8 from 4 to 3
move 1 from 1 to 3
move 6 from 6 to 1
move 1 from 3 to 6
move 5 from 1 to 7
move 10 from 5 to 9
move 3 from 9 to 8
move 7 from 6 to 2
move 1 from 7 to 8
move 3 from 5 to 8
move 3 from 6 to 2
move 6 from 8 to 9
move 1 from 5 to 3
move 2 from 3 to 1
move 2 from 4 to 8
move 6 from 6 to 9
move 1 from 1 to 4
move 17 from 9 to 2
move 1 from 4 to 1
move 2 from 7 to 8
move 1 from 9 to 8
move 3 from 8 to 4
move 3 from 1 to 4
move 9 from 8 to 2
move 1 from 8 to 4
move 12 from 2 to 7
move 4 from 7 to 4
move 1 from 8 to 1
move 10 from 4 to 2
move 3 from 3 to 2
move 1 from 9 to 7
move 11 from 7 to 3
move 1 from 3 to 1
move 2 from 3 to 9
move 1 from 3 to 7
move 2 from 1 to 9
move 1 from 6 to 5
move 7 from 3 to 6
move 1 from 7 to 3
move 3 from 3 to 4
move 1 from 5 to 7
move 2 from 4 to 3
move 2 from 4 to 8
move 1 from 7 to 6
move 2 from 6 to 8
move 1 from 9 to 2
move 1 from 9 to 5
move 1 from 5 to 1
move 1 from 8 to 6
move 1 from 3 to 2
move 4 from 6 to 1
move 5 from 1 to 4
move 11 from 2 to 4
move 2 from 8 to 2
move 1 from 8 to 9
move 27 from 2 to 5
move 4 from 6 to 3
move 3 from 2 to 4
move 2 from 5 to 9
move 1 from 5 to 7
move 2 from 9 to 5
move 14 from 4 to 7
move 2 from 4 to 7
move 3 from 4 to 8
move 4 from 3 to 1
move 4 from 1 to 8
move 2 from 3 to 9
move 2 from 9 to 3
move 7 from 8 to 9
move 1 from 3 to 8
move 2 from 3 to 2
move 25 from 5 to 9
move 1 from 5 to 8
move 1 from 8 to 7
move 26 from 9 to 1
move 23 from 1 to 5
move 7 from 9 to 7
move 1 from 9 to 8
move 1 from 9 to 2
move 5 from 7 to 1
move 20 from 5 to 6
move 1 from 7 to 6
move 2 from 5 to 3
move 1 from 8 to 6
move 21 from 6 to 8
move 1 from 6 to 4
move 1 from 1 to 7
move 2 from 1 to 6
move 1 from 1 to 3
move 1 from 2 to 5
move 1 from 2 to 6
move 2 from 7 to 6
move 6 from 7 to 9
move 3 from 1 to 2
move 17 from 8 to 1
move 1 from 4 to 1
move 2 from 6 to 9
move 3 from 8 to 9
move 2 from 3 to 7
move 2 from 9 to 8
move 4 from 7 to 3
move 4 from 3 to 4
move 2 from 5 to 8
move 4 from 8 to 4
move 3 from 6 to 8
move 18 from 1 to 5
move 1 from 3 to 4
move 3 from 2 to 4
move 5 from 9 to 1
move 10 from 7 to 5
move 5 from 1 to 3
move 5 from 3 to 5
move 5 from 4 to 3
move 2 from 4 to 2
move 5 from 8 to 3
move 25 from 5 to 2
move 3 from 3 to 6
move 1 from 1 to 3
move 3 from 6 to 7
move 1 from 4 to 2
move 1 from 5 to 8
move 2 from 4 to 9
move 1 from 8 to 1
move 20 from 2 to 7
move 10 from 7 to 1
move 1 from 1 to 7
move 4 from 7 to 8
move 5 from 5 to 4
move 4 from 8 to 6
move 1 from 1 to 3
move 5 from 7 to 4
move 2 from 1 to 5
move 4 from 9 to 1
move 3 from 2 to 5
move 5 from 5 to 1
move 1 from 9 to 1
move 11 from 1 to 3
move 1 from 6 to 2
move 7 from 3 to 5
move 11 from 3 to 7
move 1 from 2 to 6
move 7 from 7 to 8
move 1 from 9 to 1
move 2 from 3 to 1
move 1 from 5 to 3
move 4 from 1 to 6
move 4 from 6 to 3
move 9 from 4 to 5
move 2 from 8 to 2
move 4 from 6 to 9
move 3 from 2 to 4
move 1 from 8 to 6

    "
}
