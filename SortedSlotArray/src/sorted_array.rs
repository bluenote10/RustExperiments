use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter;

pub struct SortedArray<T, C>
where
    C: Fn(&T, &T) -> Ordering,
{
    spacing: usize,
    comparator: C,
    data_raw: Vec<Option<T>>,
    //data_idx: Vec<usize>,
    data_idx: HashMap<usize, usize>
}

impl<T, C> SortedArray<T, C>
where
    C: Fn(&T, &T) -> Ordering,
    T: Clone,
{
    pub fn new(comparator: C, initial_capacity: usize, spacing: usize) -> SortedArray<T, C> {
        SortedArray {
            spacing,
            comparator,
            data_raw: iter::repeat(None).take(initial_capacity * spacing).collect(),
            data_idx: HashMap::new(),
        }
    }

    pub fn insert(&mut self, t: T) {
        if self.data_idx.len() == 0 {
            if self.data_raw.len() == 0 {
                self.data_raw = iter::repeat(None).take(self.spacing).collect();
            }
            // put in middle
            let idx = self.data_raw.len() / 2;
            self.data_raw[idx] = Some(t);
            self.data_idx.insert(0, idx);
        } else {

            /*
            self.data_idx.binary_search_by(|x| {
                Ordering::Less
            });
            */
            binary_search_by(&self.data_raw, |x| (self.comparator)(x, &t));

        }
    }

}

pub enum BinarySearchResult {
    Match{idx: usize},
    Slot{idx: usize},
    Err
}

pub fn binary_search_by<T, F>(data: &[Option<T>], mut f: F) -> (usize, bool) // BinarySearchResult
where
    F: FnMut(&T) -> Ordering,
{
    if data.len() == 0 {
        //return BinarySearchResult::Err;
        return (data.len(), false);
    }
    let mut l: usize = 0;
    let mut r: usize = data.len();
    let mut equals = false;

    'outer: while r > l {
        let mut mid = l + (r - l) / 2;

        /*
        while let Some(el) = data[mid] {
            mid += 1;
            if mid == r {
                break 'outer;
            }
        }

        let cmp = f(&data[mid]);
        //let cmp = f(unsafe { self.data_raw.get_unchecked(mid) });
        base = if cmp == Greater { base } else { mid };
        size -= half;
        */

        loop {
            if let Some(el) = &data[mid] {
                let cmp = f(&el);
                match cmp {
                    Ordering::Greater => {
                        r = mid;
                    }
                    Ordering::Equal => {
                        r = mid;
                        equals = true;
                    }
                    Ordering::Less => {
                        l = mid + 1;
                    }
                }
                break;
            } else {
                mid += 1;
                if mid == r {
                    break 'outer;
                }
            }
        }
    }

    /*
    if equals {
        BinarySearchResult::Match{idx: r}
    } else {
        if r == data.len() {
            // TODO: check for slot
            BinarySearchResult::Err
        } else {
            unimplemented!()
        }

    }
    */
    (r, equals)
}

#[inline]
pub fn next<'a, T>(data: &'a [Option<T>], idx: usize, bound: usize) -> Option<(usize, &'a T)> {
    let mut i = idx;
    loop {
        if i <= bound {
            if let Some(el) = &data[i] {
                return Some((i, el));
            } else {
                i += 1;
            }
        } else {
            return None
        }
    }
}

#[inline]
pub fn prev<'a, T>(data: &'a [Option<T>], idx: usize, bound: usize) -> Option<(usize, &'a T)> {
    let mut i = idx;
    loop {
        if i >= bound {
            if let Some(el) = &data[i] {
                return Some((i, el));
            } else {
                i -= 1;
            }
        } else {
            return None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::Ordering;

    fn int_comparator(a: &i32, b: &i32) -> Ordering {
        a.cmp(b)
    }

    #[test]
    fn test_binary_search_by() {
        let data = [Some(1), Some(2), Some(3)];
        assert_eq!(binary_search_by(&data, |x| int_comparator(x, &0)), (0, false));
        assert_eq!(binary_search_by(&data, |x| int_comparator(x, &1)), (0, true));
        assert_eq!(binary_search_by(&data, |x| int_comparator(x, &2)), (1, true));
        assert_eq!(binary_search_by(&data, |x| int_comparator(x, &3)), (2, true));
        assert_eq!(binary_search_by(&data, |x| int_comparator(x, &4)), (3, false));
    }

}

/*
impl<T, C> SplaySet<T, C>
where
    C: Fn(&T, &T) -> Ordering,
{
    pub fn new(comparator: C) -> SplaySet<T, C> {
        SplaySet {
            tree: SplayTree::new(comparator),
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn clear(&mut self) {
        self.tree.clear()
    }

    pub fn contains(&self, t: &T) -> bool {
        self.tree.contains(t)
    }

    pub fn find(&self, t: &T) -> Option<&T> {
        self.tree.find_key(t)
    }

    pub fn next(&self, t: &T) -> Option<&T> {
        self.tree.next(t).map(|kv| kv.0)
    }

    pub fn prev(&self, t: &T) -> Option<&T> {
        self.tree.prev(t).map(|kv| kv.0)
    }

    pub fn insert(&mut self, t: T) -> bool {
        self.tree.insert(t, ()).is_none()
    }

    pub fn remove(&mut self, t: &T) -> bool {
        self.tree.remove(t).is_some()
    }

    pub fn min(&self) -> Option<&T> {
        self.tree.min()
    }

    pub fn max(&self) -> Option<&T> {
        self.tree.max()
    }

    pub fn traverse<F>(&self, traverse: &mut F) where F: FnMut(&T) {
        self.tree.traverse(&mut |k, _| traverse(k));
    }
}

impl<T, C> IntoIterator for SplaySet<T, C>
where
    C: Fn(&T, &T) -> Ordering,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.tree.into_iter(),
        }
    }
}

pub struct IntoIter<T> {
    inner: tree::IntoIter<T, ()>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.inner.next().map(|p| p.0)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.inner.next_back().map(|(k, _)| k)
    }
}

impl<T, C> Extend<T> for SplaySet<T, C>
where
    C: Fn(&T, &T) -> Ordering,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, i: I) {
        for t in i {
            self.insert(t);
        }
    }
}

*/