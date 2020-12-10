
pub fn preceded(target: i64, options: &[i64]) -> bool {
    for i in options {
        for j in options {
            if i != j && i + j == target {
                return true
            }
        }
    }

    false
}

pub fn first_non_sum<'a>(numbers: &'a Vec<i64>, preamble: usize) -> Option<i64> {
    for (i, v) in numbers.iter().skip(preamble).enumerate() {
        let offset = preamble + i;

        let slice = &numbers[(offset - preamble)..offset];

        if !preceded(*v, slice) { return Some(*v); }
    }

    return None
}
