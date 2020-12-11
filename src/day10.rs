use std::collections::HashMap;

pub fn optimal_config(input: &str) -> Vec<i32> {
    let mut joltages: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    // Only way we can use *all* adapters is to sort by joltage
    joltages.sort();

    // Set up initial input joltage
    joltages.insert(0, 0);
    // Set up final joltage
    joltages.push(joltages.last().unwrap() + 3);

    joltages
}

// Working backwards, fill in the possible combinations *after* the pointer
// element. Given a group of elements that looks like:
// [current, i1, i2, i3]
//
// the total possible combinations is going to be:
// * the number of combinations after i1
// * PLUS the combinations after i2 (if i2 - current is valid)
// * PLUS the combinations after i3 (if i3 - current is valid)
// * i4 - current is never valid, since that would require a joltage-step of 4
//
// Store that sum in the hash for the current element, then move to the
// prior element until we get to the beginning. The "number of configs
// after" the first element is our total possible configurations.
pub fn possible_configs<'a>(joltages: &'a Vec<i32>) -> i64 {
    // Store the number of configurations *after* the adapter at a given index
    // in the joltages array.
    let mut configs_after: Vec<i64> = Vec::with_capacity(joltages.len());

    let len = joltages.len() - 1;
    let mut pointer = len;
    // 0 possible combinations of the last element (since it isn't a pair)
    configs_after[pointer] = 0;

    // only one possible combination of the second-to-last element and the last
    // element
    pointer -= 1;
    configs_after[pointer] = 1;

    // start from the third-to-last element
    pointer -= 1;

    loop {
        let current = joltages[pointer];

        let mut num_configs = configs_after[pointer + 1];

        if pointer + 2 < len && joltages[pointer + 2] - current <= 3 {
            num_configs += configs_after[pointer + 2];
        }

        if pointer + 3 < len && joltages[pointer + 3] - current <= 3 {
            num_configs += configs_after[pointer + 3];
        }

        configs_after[pointer] = num_configs;

        if pointer == 0 { break }

        pointer = pointer - 1;
    }

    configs_after[0]
}

pub fn joltage_diffs<'a>(sorted_joltages: &'a Vec<i32>) -> (usize, usize) {
    let mut ones = 0;
    let mut threes = 0;
    for (i, v) in sorted_joltages.iter().enumerate() {
        let succ = i + 1;
        if succ >= sorted_joltages.len() { break }

        let succ_val = sorted_joltages[succ];

        match succ_val - *v {
            3 => threes += 1,
            1 => ones += 1,
            _ => ()
        }
    }

    (ones, threes)
}
