
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

pub fn numbers_adding_to<'a>(numbers: &'a Vec<i64>, target: i64) -> Option<(i64, i64)> {
    let len = numbers.len();
    // loop over each number as a "starting position", then loop over successive
    // slices of the following numbers to see if any are the sum.
    for (i, v) in numbers.iter().enumerate() {
        let start_offset = i;
        let mut sum = *v;

        let mut end_offset = start_offset + 1;

        loop {
            // once our sum exceeds the target or we crawl beyond the end of the
            // array, break the inner loop because we're not going to find what
            // we're looking for here.
            if sum > target || end_offset == len { break }

            sum += numbers[end_offset];

            if sum == target {
                let slice = &numbers[start_offset..end_offset];
                // itertools crate has a ::minmax() method that gets this
                // without the double-loop.
                let min: i64 = *slice.iter().min().unwrap();
                let max: i64 = *slice.iter().max().unwrap();
                return Some((min, max))
            // exit the inner loop early if our sum exceeds target, since it
            // will definitely continue to do so.
            } else if sum > target { break }

            end_offset += 1;
        }
    }

    None
}
