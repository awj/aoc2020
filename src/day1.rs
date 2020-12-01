use vec;

// Given an Iterator, find a pair within the iterator that sums to 2020 and
// return it. Otherwise return None.
pub fn find_pair_with_total<'a, I: Clone>(input: I) -> Option<(i32, i32)>
where
    I: Iterator<Item = i32> {
    // Not sure if this is cloning the entire data structure or just the
    // iterator. Guessing it's cloning everything. Either way, this is necessary
    // since the iterator has internal state that we'd be trying to reset on the
    // inner loop. (except the compiler prevents move-ing the iterator)
    for x in input.clone() {
        for y in input.clone() {
            if x + y == 2020 {
                return Some((x, y))
            }
        }
    }

    None
}


// Given a vector, find the triple of values in it that sum to 2020 and return
// it. Otherwise return None.
pub fn find_triple_with_total(input: &Vec<i32>) -> Option<(i32, i32, i32)> {
    // Borrowing the input as an (immutable) vector means we can just use
    // for..in to loop through it and not worry about iterators or move-ing.
    for x in input {
        for y in input {
            for z in input {
                if x + y + z == 2020 {
                    return Some((*x, *y, *z))
                }
            }
        }
    }

    None
}
