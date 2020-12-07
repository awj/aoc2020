use std::collections::HashSet;

pub fn get_sizes<'a>(input: &'a mut std::str::Lines) -> Vec<usize> {
    let mut answers: HashSet<char> = HashSet::new();
    let mut sizes: Vec<usize> = Vec::new();

    for line in input {
        if line.is_empty() {
            sizes.push(answers.len());
            answers.clear();
        } else {
            for char in line.chars() { answers.insert(char); }
        }
    }

    if !answers.is_empty() { sizes.push(answers.len()) }

    sizes
}

pub fn get_answers(input: &str) -> HashSet<char> {
    input.chars().collect()
}

pub fn all_yes<'a>(input: &'a mut std::str::Lines) -> Vec<usize> {
    let mut answers: HashSet<char> = HashSet::new();
    let mut sizes: Vec<usize> = Vec::new();
    let mut fresh_group = true;

    for line in input {
        if line.is_empty() {
            fresh_group = true;
            sizes.push(answers.len());
            answers.clear();
        } else {
            let line_answers = get_answers(line);
            if fresh_group {
                answers = line_answers;
                fresh_group = false;
            } else {
                answers = answers.intersection(&line_answers).map(|c| c.clone() ).collect();
            }
        }
    }

    if !answers.is_empty() { sizes.push(answers.len()) }

    sizes
}
