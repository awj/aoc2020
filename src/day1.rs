// Given an Iterator, find a pair within the iterator that sums to 2020 and
// return it. Otherwise return None.
pub fn find_pair_with_total<'a, I: Clone>(input: I) -> Option<(i32, i32)>
where
    I: Iterator<Item = i32> {
    // Not sure if this is cloning the entire data structure or just the
    // iterator. Guessing it's cloning the whole iterator.
    for x in input.clone() {
        for y in input.clone() {
            if x + y == 2020 {
                return Some((x, y))
            }
        }
    }

    None
}
