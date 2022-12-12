fn main() {
    println!("Part 1 is {}", part1::part1(input()));
    println!("Part 2 is {}", part2::part2(input()));
}

mod part1 {
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Monkey {
        items: Vec<u64>,
        operation: (Operand, fn(u64, u64) -> u64, Operand),
        test: u64,
        if_true: usize,
        if_false: usize,

        inspected_items: u64,
    }

    #[derive(Debug)]
    enum Operand {
        Old,
        Digit(u64),
    }

    impl Operand {
        fn value(&self, default: u64) -> u64 {
            match self {
                Operand::Old => default,
                Operand::Digit(digit) => *digit,
            }
        }
    }

    pub fn part1(input: &'static str) -> u64 {
        let mut monkeys = vec![];

        for block in input.trim().split("\n\n") {
            let lines: Vec<&str> = block.lines().map(|line| line.trim()).collect();

            let items: Vec<u64> = lines[1]
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|digit| digit.parse().unwrap())
                .collect();

            let mut operation = lines[2]
                .strip_prefix("Operation: new = ")
                .unwrap()
                .split(' ');

            let a = match operation.next().unwrap() {
                "old" => Operand::Old,
                digit => Operand::Digit(digit.parse().unwrap()),
            };
            let function = match operation.next().unwrap() {
                "*" => std::ops::Mul::mul,
                "+" => std::ops::Add::add,
                _ => panic!(),
            };
            let b = match operation.next().unwrap() {
                "old" => Operand::Old,
                digit => Operand::Digit(digit.parse().unwrap()),
            };

            let test = lines[3]
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();

            let if_true = lines[4]
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            let if_false = lines[5]
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            monkeys.push(Monkey {
                items,
                operation: (a, function, b),
                test,
                if_true,
                if_false,
                inspected_items: 0,
            });
        }

        let mut receive = HashMap::new();

        for _ in 0..20 {
            for (index, monkey) in monkeys.iter_mut().enumerate() {
                let new_items = receive.remove(&index).unwrap_or_else(Vec::new);
                monkey.items.extend(new_items);

                for item in &monkey.items {
                    monkey.inspected_items += 1;

                    let new = monkey.operation.1(
                        monkey.operation.0.value(*item),
                        monkey.operation.2.value(*item),
                    ) / 3;

                    let monkey_index = if new % monkey.test == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };

                    let entry = receive.entry(monkey_index).or_insert_with(Vec::new);
                    entry.push(new);
                }

                monkey.items = vec![];
            }
        }

        monkeys.sort_by_key(|monkey| monkey.inspected_items);

        monkeys[monkeys.len() - 1].inspected_items * monkeys[monkeys.len() - 2].inspected_items
    }
}

mod part2 {
    use std::collections::HashMap;

    // #[derive(Debug, Clone)]
    // enum ItemValue {
    //     Digit(u64),
    //     Operation(Operation),
    // }

    #[derive(Debug, Clone, Copy)]
    enum Operation {
        Square,
        Add(u64),
        Mul(u64),
    }

    #[derive(Debug)]
    struct Monkey {
        items: Vec<Vec<u64>>,
        operation: Operation,
        if_true: usize,
        if_false: usize,

        inspected_items: u64,
    }

    pub fn part2(input: &'static str) -> u64 {
        let mut monkeys = vec![];

        let mut all_modulus = vec![];

        for block in input.trim().split("\n\n") {
            let lines: Vec<&str> = block.lines().map(|line| line.trim()).collect();

            let items = lines[1]
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|digit| vec![digit.parse().unwrap()])
                .collect();

            let operation_parts: Vec<&str> = lines[2]
                .strip_prefix("Operation: new = ")
                .unwrap()
                .split(' ')
                .collect();

            let operation = match (operation_parts[0], operation_parts[1], operation_parts[2]) {
                ("old", "*", "old") => Operation::Square,
                ("old", "+", digit) => Operation::Add(digit.parse().unwrap()),
                ("old", "*", digit) => Operation::Mul(digit.parse().unwrap()),
                _ => panic!(),
            };

            let test = lines[3]
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();
            all_modulus.push(test);

            let if_true = lines[4]
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            let if_false = lines[5]
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();

            monkeys.push(Monkey {
                items,
                operation,
                if_true,
                if_false,
                inspected_items: 0,
            });
        }

        for monkey in monkeys.iter_mut() {
            for item in monkey.items.iter_mut() {
                *item = all_modulus
                    .iter()
                    .map(|modulus| item[0] % modulus)
                    .collect();
            }
        }

        let mut receive = HashMap::new();

        for _ in 0..10_000 {
            for (index, monkey) in monkeys.iter_mut().enumerate() {
                let new_items = receive.remove(&index).unwrap_or_else(Vec::new);
                monkey.items.extend(new_items);

                for item in monkey.items.iter_mut() {
                    monkey.inspected_items += 1;

                    for (monkey_index, modulus_result) in item.iter_mut().enumerate() {
                        match monkey.operation {
                            Operation::Add(digit) => {
                                *modulus_result = (*modulus_result
                                    + (digit % all_modulus[monkey_index]))
                                    % all_modulus[monkey_index]
                            }
                            Operation::Mul(digit) => {
                                *modulus_result = (*modulus_result
                                    * (digit % all_modulus[monkey_index]))
                                    % all_modulus[monkey_index]
                            }
                            Operation::Square => {
                                *modulus_result =
                                    (*modulus_result * *modulus_result) % all_modulus[monkey_index]
                            }
                        }
                    }

                    let monkey_index = if item[index] == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };

                    let entry = receive.entry(monkey_index).or_insert_with(Vec::new);

                    entry.push(item.clone());
                }

                monkey.items = vec![];
            }
        }

        monkeys.sort_by_key(|monkey| monkey.inspected_items);

        monkeys[monkeys.len() - 1].inspected_items * monkeys[monkeys.len() - 2].inspected_items
    }
}

#[test]
fn test() {
    assert_eq!(
        10605,
        part1::part1(
            "
    Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
    "
        )
    );
    assert_eq!(
        2713310158,
        part2::part2(
            "
    Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
    "
        )
    );
}

fn input() -> &'static str {
    "
    Monkey 0:
    Starting items: 98, 89, 52
    Operation: new = old * 2
    Test: divisible by 5
      If true: throw to monkey 6
      If false: throw to monkey 1

  Monkey 1:
    Starting items: 57, 95, 80, 92, 57, 78
    Operation: new = old * 13
    Test: divisible by 2
      If true: throw to monkey 2
      If false: throw to monkey 6

  Monkey 2:
    Starting items: 82, 74, 97, 75, 51, 92, 83
    Operation: new = old + 5
    Test: divisible by 19
      If true: throw to monkey 7
      If false: throw to monkey 5

  Monkey 3:
    Starting items: 97, 88, 51, 68, 76
    Operation: new = old + 6
    Test: divisible by 7
      If true: throw to monkey 0
      If false: throw to monkey 4

  Monkey 4:
    Starting items: 63
    Operation: new = old + 1
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1

  Monkey 5:
    Starting items: 94, 91, 51, 63
    Operation: new = old + 4
    Test: divisible by 13
      If true: throw to monkey 4
      If false: throw to monkey 3

  Monkey 6:
    Starting items: 61, 54, 94, 71, 74, 68, 98, 83
    Operation: new = old + 2
    Test: divisible by 3
      If true: throw to monkey 2
      If false: throw to monkey 7

  Monkey 7:
    Starting items: 90, 56
    Operation: new = old * old
    Test: divisible by 11
      If true: throw to monkey 3
      If false: throw to monkey 5
  
    "
}
