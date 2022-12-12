use std::collections::{HashMap, HashSet};

fn main() {
    println!("Part 1 is {}", compute(input(), false));
    println!("Part 2 is {}", compute(input(), true));
}

fn compute(input: &'static str, part2: bool) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut min_paths: HashMap<(i32, i32), i32> = HashMap::new();
    let mut ends = HashSet::new();

    for (x, line) in input.trim().lines().map(|line| line.trim()).enumerate() {
        for (y, mut char) in line.chars().enumerate() {
            if char == 'S' {
                char = 'a';
                ends.insert((x as i32, y as i32));
            }
            if part2 && char == 'a' {
                ends.insert((x as i32, y as i32));
            }

            if char == 'E' {
                min_paths.insert((x as i32, y as i32), 0);
                char = 'z';
            }

            map.insert((x as i32, y as i32), char.to_digit(36).unwrap() as i32);
        }
    }

    loop {
        if let Some((current_position, min_path)) = min_paths
            .iter()
            .filter(|(position, _)| !visited.contains(position))
            .min_by_key(|(_, min_path)| *min_path)
        {
            let current_position = current_position.clone();
            let min_path = *min_path;

            visited.insert(current_position.clone());
            let current_height = map.get(&current_position).unwrap();

            for (diff_x, diff_y) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let next_position = (current_position.0 + diff_x, current_position.1 + diff_y);

                if let Some(next_height) = map.get(&next_position) {
                    if current_height - next_height > 1 {
                        continue;
                    }

                    let next_min_path = min_paths.get(&next_position).unwrap_or(&i32::MAX);
                    if *next_min_path > min_path + 1 {
                        min_paths.insert(next_position, min_path + 1);
                    }
                }
            }
        } else {
            break;
        }
    }

    *min_paths
        .iter()
        .filter(|(position, _)| ends.contains(position))
        .map(|(_, min_path)| min_path)
        .min()
        .unwrap()
}

#[test]
fn test() {
    assert_eq!(
        31,
        compute(
            "
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
    ",
            false
        )
    );
    assert_eq!(394, compute(input(), false));

    assert_eq!(
        29,
        compute(
            "
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
    ",
            true
        )
    );
}

fn input() -> &'static str {
    "
    abccccccccaaaaaaaccaaaaaaaaaaaaaaaaccccccccccccccccccccccccccccccccccccaaaaaa
    abccccccccaaaaaaaccaaaaaaaaaaaaaaaaccccccccccccccccccccccccccccccccccccaaaaaa
    abccccccccccaaaaaaccaaaaaaaaaaaaaaaaccccccccccccccccacccccccccccccccccccaaaaa
    abcccccaaaacaaaaaaccaaaaaaaaaaaaaaaaacccccccccccccccaaaccccaccccccccccccccaaa
    abccccaaaaacaaccccccaaaaaacaaacaacaaaaaaacccccccccccaaaacccaacccccccccccccaaa
    abaaccaaaaaaccccaaacaaaacacaaacaaccaaaaaacccccccccccaklaccccccccccccccccccaac
    abaaccaaaaaaccaaaaaacccccccaaacccaaaaaaaccccccccccckkkllllccccccccccccccccccc
    abaaccaaaaaaccaaaaaacccccccaaaaacaaaaaaacccccccccckkkklllllcccccccaaaccaccccc
    abacccccaacccccaaaaacccccccaaaaaccaaaaaaacccccccckkkkpppllllccccccaaaaaaccccc
    abacccccccccccaaaaacccccccccaaaacccaaaaaaccccccckkkkpppppplllccccddddaaaccccc
    abccccccccccccaaaaaccccccccccaaaccaaaccccccccccckkkppppppppllllldddddddaccccc
    abccacccccccccccccccccccccccccccccaaccccccccccckkkopppupppplllmmmmdddddaacccc
    abccaaacaaaccccccccccccccccccccaaaaaaaaccccccckkkkopuuuuupppllmmmmmmddddacccc
    abccaaaaaaaccccccccccccccccccccaaaaaaaacccccjjkkkooouuuuuuppqqqqqmmmmddddcccc
    abccaaaaaacccccccccccccccaaccccccaaaacccccjjjjjjoooouuxuuuppqqqqqqmmmmdddcccc
    abcaaaaaaaacccccccccccccaaacccccaaaaaccccjjjjoooooouuuxxuuvvvvvqqqqmmmdddcccc
    abaaaaaaaaaacccccccaaaaaaacaacccaacaaacccjjjooooouuuuxxxxvvvvvvvqqqmmmdddcccc
    abaaaaaaaaaacccaaacaaaaaaaaaacccacccaaccjjjooootttuuuxxxyyvyyvvvqqqmmmeeecccc
    abcccaaacaaacccaaaaaaacaaaaaccccccccccccjjjooottttxxxxxxyyyyyyvvqqqmmmeeccccc
    abcccaaacccccccaaaaaacaaaaaccccaaccaacccjjjnnntttxxxxxxxyyyyyvvvqqqnneeeccccc
    SbccccaacccccccaaaaaaaaacaaacccaaaaaacccjjjnnntttxxxEzzzzyyyyvvqqqnnneeeccccc
    abcccccccccccccaaaaaaaaacaaccccaaaaaccccjjjnnnttttxxxxyyyyyvvvrrrnnneeecccccc
    abcccaacccccccaaaaaaaaaccccccccaaaaaacccciiinnnttttxxxyyyyywvvrrrnnneeecccccc
    abcccaaaaaaccaaaaaaaacccccccccaaaaaaaaccciiiinnnttttxyyywyyywvrrrnnneeecccccc
    abcccaaaaaaccaaaaaaaacccccccccaaaaaaaacccciiinnnntttxwywwyyywwwrrnnneeecccccc
    abcaaaaaaaccaaaaaaaaaccccccccccccaacccccccciiinnnttwwwwwwwwwwwwrrnnneeecccccc
    abcaaaaaaaccaaaaaacccccccccccccccaaccccccaaiiiinnttwwwwwwwwwwwrrrnnnffecccccc
    abcccaaaaaaccaaaaaccccccccccccccccccccaaaaaciiinnssswwwssssrwwrrrnnnfffcccccc
    abaacaaccaaccaaaccccccccaacccccccccccccaaaaaiiinnssssssssssrrrrrronnfffcccccc
    abaccaaccaacccccccccaaacaacccccccccccccaaaaaiiimmmssssssmoosrrrrooonffaaacccc
    abaaaccccaaaaaaccccccaaaaaccccccccccccaaaaaccihmmmmsssmmmoooooooooofffaaacccc
    abaaaccccaaaaaacccccccaaaaaacccccccccccccaacchhhmmmmmmmmmoooooooooffffaaccccc
    abaacccaaaaaaaccccccaaaaaaaaccccaaccccccccccchhhhmmmmmmmgggggooofffffaaaccccc
    abaacccaaaaaaaccccccaaaaaaaccccaaaaccccccccccchhhhmmmmhggggggggfffffaaaaccccc
    abccccccaaaaaaacccccaacaaaaacccaaaaccccccccccchhhhhhhhggggggggggfffaacaaccccc
    abccaacccaaaaaaccccccccaaaaaccaaaaacccccccccccchhhhhhhggaaaaaaccccccccccccccc
    abccaaaccaaccccccccccccccaaaaaaaaaccccccccccccccchhhhaaaccaaaacccccccccccccaa
    abaaaaaaaccccccccccccccccaaaaaaaaccccccccccccccccccccaaaccccaaccccccccccccaaa
    abaaaaaaaccccccccaaaccccacaaaaaacccccccccccccccccccccaaaccccccccccccccccccaaa
    abaaaaaacccccccaaaaacaaaaaaaaaaacccccccccccccccccccccaaccccccccccccccccaaaaaa
    abaaaaaacccccccaaaaaaaaaaaaaaaaaaacccccccccccccccccccccccccccccccccccccaaaaaa
    "
}
