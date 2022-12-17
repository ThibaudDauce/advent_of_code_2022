fn main() {
    println!("Part 1 is {}", part1::part1(input()));
    println!("Part 2 is {}", part2::part2(input()));
}

mod part1 {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, Clone)]
    struct Valve {
        rate: u64,
        destinations: HashMap<String, u64>,
    }

    const MAX_MINUTES: u64 = 30;

    #[derive(Debug, Clone)]
    struct Cave {
        path: Vec<String>,
        minute: u64,
        position: String,
        presure_per_minutes: u64,
        presure: u64,
        valves_names_to_visit: HashSet<String>,
    }

    pub fn part1(input: &'static str) -> u64 {
        let mut valves = HashMap::new();

        for line in input.trim().lines().map(|line| line.trim()) {
            let line = line.strip_prefix("Valve ").unwrap();
            let (name, tail) = line.split_once(" has flow rate=").unwrap();
            let (rate_as_string, tail) = tail
                .split_once("; tunnels lead to valves ")
                .unwrap_or_else(|| tail.split_once("; tunnel leads to valve ").unwrap());
            let rate: u64 = rate_as_string.parse().unwrap();
            let mut destinations: HashMap<String, u64> = tail
                .split(", ")
                .map(|destination| (destination.to_owned(), 1))
                .collect();
            destinations.insert(name.to_owned(), 0);

            valves.insert(name.to_owned(), Valve { rate, destinations });
        }

        let valves_names_to_visit: HashSet<String> = valves
            .iter()
            .filter(|(_, valve)| valve.rate != 0)
            .map(|(name, _)| name)
            .cloned()
            .collect();
        let valves_names: HashSet<String> = valves.keys().cloned().collect();

        let valves_cloned = valves.clone();

        for name in &valves_names {
            loop {
                let valve = valves.get_mut(name).unwrap();

                if valve.destinations.len() == valves_names.len() {
                    break;
                }

                for (destination, distance) in valve.destinations.clone().iter() {
                    let other = valves_cloned.get(destination).unwrap();
                    for (other_destination, other_distance) in &other.destinations {
                        if let Some(previous_distance) = valve.destinations.get(other_destination) {
                            assert!(*previous_distance <= distance + other_distance);
                        } else {
                            valve
                                .destinations
                                .insert(other_destination.clone(), distance + other_distance);
                        }

                        if !valve.destinations.contains_key(other_destination) {}
                    }
                }
            }
        }

        let cave = Cave {
            path: vec![],
            minute: 1,
            position: "AA".to_owned(),
            presure_per_minutes: 0,
            presure: 0,
            valves_names_to_visit,
        };
        let mut caves = vec![cave];
        loop {
            let mut changed = false;
            let mut new_caves = vec![];

            for cave in caves.into_iter() {
                let (next, did_change) = next_caves(&valves, cave);
                new_caves.extend(next);

                if did_change {
                    changed = true;
                }
            }

            caves = new_caves;

            if !changed {
                break;
            }
        }

        let best_cave = caves
            .iter()
            .max_by_key(|cave| cave.presure)
            .unwrap()
            .clone();

        best_cave.presure
    }

    fn next_caves(valves: &HashMap<String, Valve>, mut cave: Cave) -> (Vec<Cave>, bool) {
        if cave.minute > MAX_MINUTES || cave.valves_names_to_visit.is_empty() {
            if cave.minute <= MAX_MINUTES {
                cave.presure += (MAX_MINUTES - cave.minute + 1) * cave.presure_per_minutes;
                cave.minute = MAX_MINUTES + 1;
            }

            return (vec![cave], false);
        }

        let mut next = Vec::with_capacity(cave.valves_names_to_visit.len());
        for valve_name_to_visit in cave.valves_names_to_visit.clone() {
            let mut new_cave = cave.clone();

            let number_of_minutes_passing = valves
                .get(&new_cave.position)
                .unwrap()
                .destinations
                .get(&valve_name_to_visit)
                .unwrap()
                + 1;

            if new_cave.minute + number_of_minutes_passing > MAX_MINUTES {
                continue;
            }

            new_cave.minute += number_of_minutes_passing;
            new_cave.presure += new_cave.presure_per_minutes * number_of_minutes_passing;
            new_cave.presure_per_minutes += valves.get(&valve_name_to_visit).unwrap().rate;
            new_cave.path.push(new_cave.position);
            new_cave.position = valve_name_to_visit.clone();

            new_cave
                .valves_names_to_visit
                .remove(&valve_name_to_visit.clone());

            next.push(new_cave)
        }

        if next.is_empty() {
            if cave.minute <= MAX_MINUTES {
                cave.presure += (MAX_MINUTES - cave.minute + 1) * cave.presure_per_minutes;
                cave.minute = MAX_MINUTES + 1;
            }

            return (vec![cave], false);
        }

        (next, true)
    }
}

mod part2 {
    use std::{
        cmp::max,
        collections::{HashMap, HashSet},
    };

    #[derive(Debug, Clone)]
    struct Valve {
        rate: u64,
        destinations: HashMap<String, u64>,
    }

    const MAX_MINUTES: u64 = 26;

    #[derive(Debug, Clone)]
    enum Action {
        Nothing,
        Done,
        Go(u64, String),
    }

    #[derive(Debug, Clone)]
    struct Cave {
        path: (Vec<String>, Vec<String>),
        minute: u64,
        position: (String, String),
        action: (Action, Action),
        presure_per_minutes: u64,
        presure: u64,
        valves_names_to_visit: HashSet<String>,
    }

    pub fn part2(input: &'static str) -> u64 {
        let mut valves = HashMap::new();

        for line in input.trim().lines().map(|line| line.trim()) {
            let line = line.strip_prefix("Valve ").unwrap();
            let (name, tail) = line.split_once(" has flow rate=").unwrap();
            let (rate_as_string, tail) = tail
                .split_once("; tunnels lead to valves ")
                .unwrap_or_else(|| tail.split_once("; tunnel leads to valve ").unwrap());
            let rate: u64 = rate_as_string.parse().unwrap();
            let mut destinations: HashMap<String, u64> = tail
                .split(", ")
                .map(|destination| (destination.to_owned(), 1))
                .collect();
            destinations.insert(name.to_owned(), 0);

            valves.insert(name.to_owned(), Valve { rate, destinations });
        }

        let valves_names_to_visit: HashSet<String> = valves
            .iter()
            .filter(|(_, valve)| valve.rate != 0)
            .map(|(name, _)| name)
            .cloned()
            .collect();
        let valves_names: HashSet<String> = valves.keys().cloned().collect();

        let valves_cloned = valves.clone();

        for name in &valves_names {
            loop {
                let valve = valves.get_mut(name).unwrap();

                if valve.destinations.len() == valves_names.len() {
                    break;
                }

                for (destination, distance) in valve.destinations.clone().iter() {
                    let other = valves_cloned.get(destination).unwrap();
                    for (other_destination, other_distance) in &other.destinations {
                        if let Some(previous_distance) = valve.destinations.get(other_destination) {
                            assert!(*previous_distance <= distance + other_distance);
                        } else {
                            valve
                                .destinations
                                .insert(other_destination.clone(), distance + other_distance);
                        }

                        if !valve.destinations.contains_key(other_destination) {}
                    }
                }
            }
        }

        let cave = Cave {
            path: (vec![], vec![]),
            minute: 1,
            position: ("AA".to_owned(), "AA".to_owned()),
            action: (Action::Nothing, Action::Nothing),
            presure_per_minutes: 0,
            presure: 0,
            valves_names_to_visit,
        };

        let best_cave = best_cave(&valves, cave, 0);
        // let mut caves = vec![cave];
        // loop {
        //     let mut changed = false;
        //     let mut new_caves = vec![];

        //     for cave in caves.into_iter() {
        //         let (next, did_change) = next_caves(&valves, cave);
        //         new_caves.extend(next);

        //         if did_change {
        //             changed = true;
        //         }
        //     }

        //     caves = new_caves;

        //     if !changed {
        //         break;
        //     }
        // }

        // let best_cave = caves
        //     .iter()
        //     .max_by_key(|cave| cave.presure)
        //     .unwrap()
        //     .clone();

        dbg!(&best_cave);

        best_cave.presure
    }

    fn best_cave(valves: &HashMap<String, Valve>, mut cave: Cave, mut current_score: u64) -> Cave {
        let mut valves_to_visit: Vec<String> = cave.valves_names_to_visit.iter().cloned().collect();
        valves_to_visit.sort_by_key(|name| valves.get(name).unwrap().rate as i64 * -1);

        let minutes_remaining = MAX_MINUTES - cave.minute + 2;
        let best_theory_score = valves_to_visit
            .iter()
            .enumerate()
            .map(|(index, name)| {
                max(
                    0,
                    (minutes_remaining as i64 - index as i64)
                        * valves.get(name).unwrap().rate as i64,
                ) as u64
            })
            .sum::<u64>()
            + cave.presure
            + minutes_remaining * cave.presure_per_minutes;

        if current_score > best_theory_score {
            // println!("Minutes remaining {minutes_remaining}");
            // println!("Theory score {best_theory_score}");
            // println!("{current_score} / {best_theory_score}");
            return cave;
        }

        match cave.action {
            (Action::Done, Action::Done) => {
                if cave.minute <= MAX_MINUTES {
                    cave.presure += (MAX_MINUTES - cave.minute + 1) * cave.presure_per_minutes;
                    cave.minute = MAX_MINUTES + 1;
                }

                return cave;
            }
            (Action::Go(minutes_a, destination_a), Action::Go(minutes_b, destination_b)) => {
                if minutes_a == minutes_b {
                    cave.minute += minutes_a;
                    assert!(cave.minute <= MAX_MINUTES);
                    cave.presure += cave.presure_per_minutes * minutes_a;

                    cave.presure_per_minutes += valves.get(&destination_a).unwrap().rate;
                    cave.presure_per_minutes += valves.get(&destination_b).unwrap().rate;

                    cave.path.0.push(destination_a.clone());
                    cave.path.1.push(destination_b.clone());

                    cave.position.0 = destination_a;
                    cave.position.1 = destination_b;

                    cave.action.0 = Action::Nothing;
                    cave.action.1 = Action::Nothing;
                } else if minutes_a < minutes_b {
                    cave.minute += minutes_a;
                    assert!(cave.minute <= MAX_MINUTES);
                    cave.presure += cave.presure_per_minutes * minutes_a;

                    cave.presure_per_minutes += valves.get(&destination_a).unwrap().rate;
                    cave.path.0.push(destination_a.clone());
                    cave.position.0 = destination_a;

                    cave.action.0 = Action::Nothing;
                    cave.action.1 = Action::Go(minutes_b - minutes_a, destination_b);
                } else {
                    cave.minute += minutes_b;
                    assert!(cave.minute <= MAX_MINUTES);
                    cave.presure += cave.presure_per_minutes * minutes_b;

                    cave.presure_per_minutes += valves.get(&destination_b).unwrap().rate;
                    cave.path.1.push(destination_b.clone());
                    cave.position.1 = destination_b;

                    cave.action.0 = Action::Go(minutes_a - minutes_b, destination_a);
                    cave.action.1 = Action::Nothing;
                }
            }
            (Action::Go(minutes_a, destination_a), Action::Done) => {
                cave.minute += minutes_a;
                assert!(cave.minute <= MAX_MINUTES);
                cave.presure += cave.presure_per_minutes * minutes_a;

                cave.presure_per_minutes += valves.get(&destination_a).unwrap().rate;
                cave.path.0.push(destination_a.clone());
                cave.position.0 = destination_a;

                cave.action.0 = Action::Nothing;
            }
            (Action::Done, Action::Go(minutes_b, destination_b)) => {
                cave.minute += minutes_b;
                assert!(cave.minute <= MAX_MINUTES);
                cave.presure += cave.presure_per_minutes * minutes_b;

                cave.presure_per_minutes += valves.get(&destination_b).unwrap().rate;
                cave.path.1.push(destination_b.clone());
                cave.position.1 = destination_b;

                cave.action.1 = Action::Nothing;
            }
            _ => {}
        }

        let position = match cave.action {
            (Action::Nothing, Action::Nothing) => &cave.position.0,
            (Action::Go(..) | Action::Done, Action::Nothing) => &cave.position.1,
            (Action::Nothing, Action::Go(..) | Action::Done) => &cave.position.0,
            _ => panic!(),
        };

        valves_to_visit.sort_by_key(|name| {
            (MAX_MINUTES as i64
                - *valves
                    .get(position)
                    .unwrap()
                    .destinations
                    .get(name)
                    .unwrap() as i64)
                * valves.get(name).unwrap().rate as i64
                * -1
        });

        let mut current_best_cave = None;

        for valve_name_to_visit in valves_to_visit {
            let mut new_cave = cave.clone();

            let (position, action) = match new_cave.action {
                (Action::Nothing, Action::Nothing) => {
                    (&mut new_cave.position.0, &mut new_cave.action.0)
                }
                (Action::Go(..) | Action::Done, Action::Nothing) => {
                    (&mut new_cave.position.1, &mut new_cave.action.1)
                }
                (Action::Nothing, Action::Go(..) | Action::Done) => {
                    (&mut new_cave.position.0, &mut new_cave.action.0)
                }
                _ => panic!(),
            };

            let number_of_minutes_passing = valves
                .get(position)
                .unwrap()
                .destinations
                .get(&valve_name_to_visit)
                .unwrap()
                + 1;

            if new_cave.minute + number_of_minutes_passing > MAX_MINUTES {
                continue;
            }

            *action = Action::Go(number_of_minutes_passing, valve_name_to_visit.clone());

            new_cave
                .valves_names_to_visit
                .remove(&valve_name_to_visit.clone());

            let new_best_cave = best_cave(valves, new_cave, current_score);

            if new_best_cave.presure > current_score {
                current_score = new_best_cave.presure;
                current_best_cave = Some(new_best_cave);
            }
        }

        if let Some(current_best_cave) = current_best_cave {
            current_best_cave
        } else {
            match cave.action {
                (Action::Nothing, Action::Nothing) => cave.action.0 = Action::Done,
                (Action::Go(..) | Action::Done, Action::Nothing) => cave.action.1 = Action::Done,
                (Action::Nothing, Action::Go(..) | Action::Done) => cave.action.0 = Action::Done,
                _ => panic!(),
            }

            let new_presure = cave.presure;
            best_cave(valves, cave, max(current_score, new_presure))
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        1651,
        part1::part1(
            "
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II
    
    "
        )
    );
    assert_eq!(
        1707,
        part2::part2(
            "
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II
    
    "
        )
    );
}

fn input() -> &'static str {
    "
    Valve XG has flow rate=0; tunnels lead to valves CR, OH
Valve ZF has flow rate=7; tunnels lead to valves SC, BY, PM, LW, CJ
Valve RD has flow rate=13; tunnels lead to valves JS, VM
Valve XJ has flow rate=0; tunnels lead to valves JO, YO
Valve CJ has flow rate=0; tunnels lead to valves UA, ZF
Valve UA has flow rate=0; tunnels lead to valves ZP, CJ
Valve EQ has flow rate=0; tunnels lead to valves ZP, RP
Valve IU has flow rate=0; tunnels lead to valves EV, CN
Valve QM has flow rate=0; tunnels lead to valves XA, CN
Valve WC has flow rate=0; tunnels lead to valves JS, OH
Valve MU has flow rate=0; tunnels lead to valves AA, ZP
Valve MW has flow rate=11; tunnels lead to valves BM, AG, RG, NL
Valve XA has flow rate=0; tunnels lead to valves JO, QM
Valve OH has flow rate=12; tunnels lead to valves WC, YS, XG, KO
Valve QD has flow rate=20; tunnels lead to valves BY, KY, CR, RP
Valve OE has flow rate=0; tunnels lead to valves FB, BU
Valve CB has flow rate=0; tunnels lead to valves AA, FX
Valve TB has flow rate=23; tunnel leads to valve VM
Valve PM has flow rate=0; tunnels lead to valves ZF, AA
Valve YS has flow rate=0; tunnels lead to valves OH, RG
Valve KO has flow rate=0; tunnels lead to valves OH, VT
Valve AA has flow rate=0; tunnels lead to valves PM, MU, BM, AW, CB
Valve RG has flow rate=0; tunnels lead to valves YS, MW
Valve BU has flow rate=0; tunnels lead to valves OE, EV
Valve RK has flow rate=0; tunnels lead to valves KY, FX
Valve JO has flow rate=18; tunnels lead to valves NL, SX, XA, XJ
Valve AG has flow rate=0; tunnels lead to valves IS, MW
Valve AW has flow rate=0; tunnels lead to valves BS, AA
Valve ZP has flow rate=9; tunnels lead to valves UA, NG, DU, MU, EQ
Valve KY has flow rate=0; tunnels lead to valves QD, RK
Valve EV has flow rate=19; tunnels lead to valves VT, BU, IU, SX
Valve FB has flow rate=24; tunnel leads to valve OE
Valve DU has flow rate=0; tunnels lead to valves IS, ZP
Valve NG has flow rate=0; tunnels lead to valves FX, ZP
Valve HC has flow rate=0; tunnels lead to valves CN, HB
Valve SC has flow rate=0; tunnels lead to valves IS, ZF
Valve HB has flow rate=22; tunnel leads to valve HC
Valve VM has flow rate=0; tunnels lead to valves RD, TB
Valve LW has flow rate=0; tunnels lead to valves ZF, FX
Valve SX has flow rate=0; tunnels lead to valves EV, JO
Valve FX has flow rate=6; tunnels lead to valves FA, NG, RK, LW, CB
Valve JS has flow rate=0; tunnels lead to valves WC, RD
Valve BM has flow rate=0; tunnels lead to valves MW, AA
Valve FA has flow rate=0; tunnels lead to valves IS, FX
Valve RP has flow rate=0; tunnels lead to valves QD, EQ
Valve NL has flow rate=0; tunnels lead to valves MW, JO
Valve CN has flow rate=15; tunnels lead to valves HC, QM, IU
Valve BS has flow rate=0; tunnels lead to valves IS, AW
Valve KP has flow rate=25; tunnel leads to valve YO
Valve YO has flow rate=0; tunnels lead to valves XJ, KP
Valve CR has flow rate=0; tunnels lead to valves XG, QD
Valve BY has flow rate=0; tunnels lead to valves QD, ZF
Valve IS has flow rate=5; tunnels lead to valves DU, SC, AG, FA, BS
Valve VT has flow rate=0; tunnels lead to valves KO, EV

    "
}
