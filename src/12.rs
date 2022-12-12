use std::collections::{HashMap, HashSet};

fn main() {
    println!("Part 1 is {}", part1(input()));
}

fn part1(input: &'static str) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut min_paths: HashMap<(i32, i32), i32> = HashMap::new();
    let mut end = None;

    for (x, line) in input.trim().lines().map(|line| line.trim()).enumerate() {
        for (y, mut char) in line.chars().enumerate() {
            if char == 'S' {
                min_paths.insert((x as i32, y as i32), 0);
                char = 'a';
            }
            if char == 'E' {
                end = Some((x as i32, y as i32));
                char = 'z';
            }

            map.insert((x as i32, y as i32), char.to_digit(36).unwrap() as i32);
        }
    }

    let end = end.unwrap();

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
                    if next_height - current_height > 1 {
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

    *min_paths.get(&end).unwrap()
}

#[test]
fn test() {
    assert_eq!(
        31,
        part1(
            "
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
    "
        )
    )
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
