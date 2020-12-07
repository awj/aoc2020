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
