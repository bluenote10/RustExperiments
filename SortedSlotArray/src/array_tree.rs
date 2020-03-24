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

        // Binary search for block index
        let (idx_block, equals) = binary_search_by(
            &self.data,
            |block| (self.comparator)(&block[0], &t),
        );
        if equals {
            return false;
        }

        // Convert from "first larger" to "last smaller" index semantics
        let mut idx_block = if idx_block > 0 {
            idx_block - 1
        } else {
            0
        };

        // Split block if necessary
        if self.data[idx_block].len() >= self.capacity as usize {
            let tail_from = (self.capacity / 2) as usize;
            let tail_upto = self.capacity as usize;
            let block_tail = self.data[idx_block][tail_from .. tail_upto].to_vec();

            self.data[idx_block].truncate(tail_from);
            self.data.insert(idx_block + 1, block_tail);

            // Determine into which of the two split blocks the new value goes.
            // FIXME: Can we miss an "equals" case here if we go into block than doesn't have the equal element?
            if (self.comparator)(&t, &self.data[idx_block + 1][0]) == Ordering::Greater {
                idx_block += 1;
            }
        }

        // Binary search for value index
        let (idx_value, equals) = binary_search_by(
            &self.data[idx_block],
            |x| (self.comparator)(&x, &t),
        );
        if equals {
            return false;
        }

        // Value insert
        let block_len = self.data[idx_block].len();
        if idx_block < block_len {
            self.data[idx_block].insert(idx_value, t);
        } else {
            self.data[idx_block].push(t);
        }

        self.num_elements += 1;
        true
    }

    pub fn traverse<F>(&self, mut f: F)
    where
        F: FnMut(usize, &T),
    {
        let mut i = 0;
        for block in &self.data {
            for x in block {
                f(i, x);
                i += 1;
            }
        }
    }

    pub fn collect(&self) -> Vec<T> {
        let mut data = Vec::with_capacity(self.num_elements);
        self.traverse(|_, x| data.push(x.clone()));
        data
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
    macro_rules! insert_many {
        ($at:expr, $data:expr) => {
            for x in $data.iter() {
                $at.insert(x.clone());
            }
        };
    }

    #[test]
    fn test_array_tree_prefers_push() {
        let mut at = new_array!(16, vec![vec![1, 2], vec![4, 5]]);
        assert_eq!(at.num_elements, 4);
        at.insert(3);
        assert_eq!(at.data, [vec![1, 2, 3], vec![4, 5]]);
        assert_eq!(at.num_elements, 5);
    }

    #[test]
    fn test_array_tree_split() {
        let mut at = new_array!(2, vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(at.num_elements, 4);
        at.insert(1);
        assert_eq!(at.data, [vec![1, 2], vec![4], vec![6, 8]]);
        assert_eq!(at.num_elements, 5);

        let mut at = new_array!(2, vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(at.num_elements, 4);
        at.insert(3);
        assert_eq!(at.data, [vec![2, 3], vec![4], vec![6, 8]]);
        assert_eq!(at.num_elements, 5);

        let mut at = new_array!(2, vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(at.num_elements, 4);
        at.insert(5);
        assert_eq!(at.data, [vec![2], vec![4, 5], vec![6, 8]]);
        assert_eq!(at.num_elements, 5);

        let mut at = new_array!(2, vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(at.num_elements, 4);
        at.insert(7);
        assert_eq!(at.data, [vec![2, 4], vec![6, 7], vec![8]]);
        assert_eq!(at.num_elements, 5);

        let mut at = new_array!(2, vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(at.num_elements, 4);
        at.insert(9);
        assert_eq!(at.data, [vec![2, 4], vec![6], vec![8, 9]]);
        assert_eq!(at.num_elements, 5);
    }

    #[test]
    fn test_array_tree_collect() {
        for cap in vec![2, 3, 4, 5] {
            let mut at = ArrayTree::new(int_comparator, cap as u16);
            insert_many!(at, [1, 2, 3, 4]);
            assert_eq!(at.collect(), [1, 2, 3, 4]);

            let mut at = ArrayTree::new(int_comparator, cap as u16);
            insert_many!(at, [1, 2, 3, 4]);
            assert_eq!(at.collect(), [1, 2, 3, 4]);
        }
    }
}