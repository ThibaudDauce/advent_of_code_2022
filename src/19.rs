use std::{
    cmp::max,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    println!("Part 1 is {}", part1(input()));
    println!("Part 2 is {}", part2(input()));
}

#[derive(Debug)]
struct Blueprint {
    id: usize,

    cost_ore_robot_in_ore: usize,

    cost_clay_robot_in_ore: usize,

    cost_obsidian_robot_in_ore: usize,
    cost_obsidian_robot_in_clay: usize,

    cost_geode_robot_in_ore: usize,
    cost_geode_robot_in_obsidian: usize,

    max_ore_needed: usize,
    max_clay_needed: usize,
    max_obsidian_needed: usize,
}

#[derive(Debug, Clone)]
struct State {
    minute: usize,

    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,

    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

fn part1(input: &'static str) -> usize {
    let mut sum = 0;
    for line in input.trim().lines().map(|line| line.trim()) {
        let (blueprint_id, max) = max_geode(line, 24);

        println!("{blueprint_id} max is {max}");
        sum += max * blueprint_id;
    }

    sum
}

fn part2(input: &'static str) -> usize {
    let mul = Arc::new(Mutex::new(1));
    let mut handles = vec![];
    for line in input.trim().lines().map(|line| line.trim()).take(3) {
        let mul = Arc::clone(&mul);
        let handle = thread::spawn(move || {
            let (blueprint_id, max) = max_geode(line, 32);

            println!("{blueprint_id} max is {max}");
            let mut mul = mul.lock().unwrap();
            *mul *= max;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = mul.lock().unwrap();
    *result
}

fn max_geode(line: &str, max_minutes: usize) -> (usize, usize) {
    let (id, tail) = line.split_once(": ").unwrap();
    let id = id.strip_prefix("Blueprint ").unwrap().parse().unwrap();

    let words: Vec<_> = tail.split(' ').collect();

    let cost_ore_robot_in_ore = words[4].parse().unwrap();
    let cost_clay_robot_in_ore = words[10].parse().unwrap();
    let cost_obsidian_robot_in_ore = words[16].parse().unwrap();
    let cost_obsidian_robot_in_clay = words[19].parse().unwrap();
    let cost_geode_robot_in_ore = words[25].parse().unwrap();
    let cost_geode_robot_in_obsidian = words[28].parse().unwrap();

    let blueprint = Blueprint {
        id,
        cost_ore_robot_in_ore,

        cost_clay_robot_in_ore,

        cost_obsidian_robot_in_ore,
        cost_obsidian_robot_in_clay,

        cost_geode_robot_in_ore,
        cost_geode_robot_in_obsidian,

        max_ore_needed: max(
            max(
                max(cost_ore_robot_in_ore, cost_clay_robot_in_ore),
                cost_obsidian_robot_in_ore,
            ),
            cost_geode_robot_in_ore,
        ),
        max_clay_needed: cost_obsidian_robot_in_clay,
        max_obsidian_needed: cost_geode_robot_in_obsidian,
    };

    let state = State {
        minute: 1,

        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,

        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };

    (blueprint.id, compute(&blueprint, state, max_minutes, 0))
}

fn compute(blueprint: &Blueprint, mut state: State, max_minutes: usize, mut max: usize) -> usize {
    if state.minute == (max_minutes + 1) {
        return state.geode;
    }

    let mut theory_state = state.clone();
    for _ in state.minute..(max_minutes + 1) {
        if theory_state.obsidian > blueprint.cost_geode_robot_in_obsidian {
            theory_state.geode_robots += 1;
        } else if theory_state.clay > blueprint.cost_obsidian_robot_in_clay {
            theory_state.obsidian_robots += 1;
        } else {
            theory_state.clay_robots += 1;
        }

        theory_state.clay += theory_state.clay_robots;
        theory_state.obsidian += theory_state.obsidian_robots;
        theory_state.geode += theory_state.geode_robots;
    }

    if theory_state.geode <= max {
        // println!("Skip {}/{}", theory_state.geode, max);
        return max;
    }

    let can_create_geode_robot = state.obsidian >= blueprint.cost_geode_robot_in_obsidian
        && state.ore >= blueprint.cost_geode_robot_in_ore;

    let can_create_obsidian_robot = state.clay >= blueprint.cost_obsidian_robot_in_clay
        && state.ore >= blueprint.cost_obsidian_robot_in_ore
        && blueprint.max_obsidian_needed > state.obsidian_robots;

    let can_create_clay_robot = state.ore >= blueprint.cost_clay_robot_in_ore
        && blueprint.max_clay_needed > state.clay_robots;

    let can_create_ore_robot =
        state.ore >= blueprint.cost_ore_robot_in_ore && blueprint.max_ore_needed > state.ore_robots;

    state.minute += 1;
    state.ore += state.ore_robots;
    state.clay += state.clay_robots;
    state.obsidian += state.obsidian_robots;
    state.geode += state.geode_robots;

    if can_create_geode_robot {
        let mut new_state = state.clone();
        new_state.geode_robots += 1;
        new_state.obsidian -= blueprint.cost_geode_robot_in_obsidian;
        new_state.ore -= blueprint.cost_geode_robot_in_ore;

        let new_max = compute(blueprint, new_state, max_minutes, max);
        if new_max > max {
            // println!("{}: New max is {new_max}", blueprint.id);
            max = new_max;
        }
    }

    if can_create_obsidian_robot {
        let mut new_state = state.clone();
        new_state.obsidian_robots += 1;
        new_state.clay -= blueprint.cost_obsidian_robot_in_clay;
        new_state.ore -= blueprint.cost_obsidian_robot_in_ore;

        let new_max = compute(blueprint, new_state, max_minutes, max);
        if new_max > max {
            // println!("{}: New max is {new_max}", blueprint.id);
            max = new_max;
        }
    }

    if can_create_clay_robot {
        let mut new_state = state.clone();
        new_state.clay_robots += 1;
        new_state.ore -= blueprint.cost_clay_robot_in_ore;

        let new_max = compute(blueprint, new_state, max_minutes, max);
        if new_max > max {
            // println!("{}: New max is {new_max}", blueprint.id);
            max = new_max;
        }
    }

    {
        let new_state = state.clone();
        let new_max = compute(blueprint, new_state, max_minutes, max);
        if new_max > max {
            // println!("{}: New max is {new_max}", blueprint.id);
            max = new_max;
        }
    }

    if can_create_ore_robot {
        let mut new_state = state.clone();
        new_state.ore_robots += 1;
        new_state.ore -= blueprint.cost_ore_robot_in_ore;

        let new_max = compute(blueprint, new_state, max_minutes, max);
        if new_max > max {
            max = new_max;
        }
    }

    max
}

#[test]
fn test() {
    assert_eq!(
        33,
        part1(
            "
                Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
                Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "
        )
    );
    assert_eq!(
        62 * 56,
        part2(
            "
                Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
                Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "
        )
    );
}

fn input() -> &'static str {
    "
    Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 18 clay. Each geode robot costs 4 ore and 19 obsidian.
    Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 4 ore and 12 obsidian.
    Blueprint 3: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 3 ore and 8 obsidian.
    Blueprint 4: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 4 ore and 13 obsidian.
    Blueprint 5: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 3 ore and 10 obsidian.
    Blueprint 6: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 4 ore and 8 obsidian.
    Blueprint 7: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 2 ore and 7 obsidian.
    Blueprint 8: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 6 clay. Each geode robot costs 3 ore and 16 obsidian.
    Blueprint 9: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 3 ore and 17 obsidian.
    Blueprint 10: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 9 clay. Each geode robot costs 4 ore and 16 obsidian.
    Blueprint 11: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 4 ore and 18 obsidian.
    Blueprint 12: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 12 obsidian.
    Blueprint 13: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 5 clay. Each geode robot costs 3 ore and 18 obsidian.
    Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 4 ore and 11 obsidian.
    Blueprint 15: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    Blueprint 16: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 3 ore and 20 obsidian.
    Blueprint 17: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 2 ore and 11 obsidian.
    Blueprint 18: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 3 ore and 11 obsidian.
    Blueprint 19: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 17 obsidian.
    Blueprint 20: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 7 obsidian.
    Blueprint 21: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 4 ore and 17 obsidian.
    Blueprint 22: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 15 clay. Each geode robot costs 2 ore and 13 obsidian.
    Blueprint 23: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 3 ore and 14 obsidian.
    Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 10 obsidian.
    Blueprint 25: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 20 obsidian.
    Blueprint 26: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 2 ore and 16 obsidian.
    Blueprint 27: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 10 obsidian.
    Blueprint 28: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 2 ore and 12 obsidian.
    Blueprint 29: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 15 obsidian.
    Blueprint 30: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 2 ore and 9 obsidian.
    "
}
