use std::cmp::Ordering;

pub struct ArrayTree<T, C>
where
    C: Fn(&T, &T) -> Ordering,
{
    comparator: C,
    data: Vec<Vec<T>>,
    capacity: u16,
    num_elements: usize,
}

impl<T, C> ArrayTree<T, C>
where
    C: Fn(&T, &T) -> Ordering,
    T: Clone + std::fmt::Debug,
{
    pub fn new(comparator: C, capacity: u16) -> ArrayTree<T, C> {
        let data = Vec::with_capacity(capacity as usize);
        //data.push(Vec::with_capacity(capacity as usize));
        ArrayTree {
            comparator,
            data,
            capacity,
            num_elements: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.num_elements
    }

    pub fn insert(&mut self, t: T) -> bool {
        if self.data.len() == 0 {
            self.data.push(self.new_block(t));
            return true;
        }

        let (block_idx_first_larger_or_equals, equals) = binary_search_by(
            &self.data,
            |block| (self.comparator)(&block[0], &t),
        );
        if equals {
            return false;
        }

        let block_idx_last_smaller = if block_idx_first_larger_or_equals > 0 {
            block_idx_first_larger_or_equals - 1
        } else {
            0
        };

        let (value_idx_first_larger_or_equals, equals) = binary_search_by(
            &self.data[block_idx_last_smaller],
            |x| (self.comparator)(&x, &t),
        );
        if equals {
            return false;
        }

        if block_idx_last_smaller < self.data[block_idx_last_smaller].len() {
            self.data[block_idx_last_smaller].insert(value_idx_first_larger_or_equals, t);
        } else {
            self.data[block_idx_last_smaller].push(t);
        }

        true
    }

    pub fn debug(&self) {
        println!("{:?}", self.data);
    }

    fn new_block(&self, t: T) -> Vec<T> {
        let mut block = Vec::with_capacity(self.capacity as usize);
        block.push(t);
        block
    }
}


pub fn binary_search_by<T, F>(data: &[T], mut f: F) -> (usize, bool)
where
    F: FnMut(&T) -> Ordering,
    T: std::fmt::Debug,
{
    if data.len() == 0 {
        return (data.len(), false);
    }
    let mut l: usize = 0;
    let mut r: usize = data.len();

    while r > l {
        let mid = l + (r - l) / 2;

        let mid_el = &data[mid];
        // println!("{} {} {} {:?}", l, r, mid, mid_el);

        let cmp = f(mid_el);
        match cmp {
            Ordering::Greater => {
                r = mid;
            }
            Ordering::Equal => {
                return (mid, true)
            }
            Ordering::Less => {
                l = mid + 1;
            }
        }
    }

    (r, false)
}


/*
pub fn find_last_block_smaller<T, F>(data: &[Vec<T>], mut f: F) -> (usize, bool)
where
    F: FnMut(&T) -> Ordering,
    T: std::fmt::Debug,
{
    (0, false)
}

pub fn find_insert_index<T, F>(data: &[Vec<T>], mut f: F) -> (usize, bool)
where
    F: FnMut(&T) -> Ordering,
    T: std::fmt::Debug,
{
    (0, false)
}
*/

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::Ordering;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    fn int_comparator(a: &i32, b: &i32) -> Ordering {
        a.cmp(b)
    }

    macro_rules! new_array {
        ($capacity:expr, $data:expr) => {{
            let data: Vec<Vec<i32>> = $data;
            let num_elements = data.iter().map(|block| block.len()).sum();
            ArrayTree {
                comparator: int_comparator,
                capacity: $capacity,
                data: $data,
                num_elements,
            }
        }};
    }

    #[test]
    fn test_array_tree() {
        let mut at = ArrayTree::new(int_comparator, 32);
        at.insert(0);
        at.debug();
        at.insert(1);
        at.debug();
        at.insert(-11);
        at.debug();
        let mut at = new_array!(2, vec![vec![1, 2], vec![4, 5]]);
        assert_eq!(at.num_elements, 4);
        at.debug();
    }
}