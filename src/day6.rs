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

    // I *really* need a better solution for this "group input by new line
    // separators" thing. It's super annoying to have to keep checking edge
    // cases. (IOW I didn't here after getting it right for the first star...)
    for line in input {
        if line.is_empty() {
            fresh_group = true;
            sizes.push(answers.len());
            answers.clear();
        } else {
            if fresh_group {
                answers = get_answers(line);
                fresh_group = false;
            } else {
                // Kinda weird that there's no reduce-to-intersection method for
                // a HashSet. Instead we end up having to allocate new sets over
                // and over.
                answers = answers.intersection(&line_answers).map(|c| c.clone() ).collect();
            }
        }
    }

    if !answers.is_empty() { sizes.push(answers.len()) }

    sizes
}
