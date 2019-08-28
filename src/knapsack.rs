use std::cmp::max;

pub trait Item {
    /// Returns the value.
    fn value(&self) -> u64;

    /// Returns the weight.
    fn weight(&self) -> u64;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Calc<'a, T> {
    Yet { items: &'a [T], limit: u64 },
    Max(u64, u64),
}

impl<'a, T> Calc<'a, T> {
    #[inline]
    fn yet(items: &'a [T], limit: u64) -> Calc<'a, T> {
        Calc::Yet { items: items, limit: limit }
    }

    #[inline]
    fn as_max_mut(&mut self) -> Option<(&mut u64, &mut u64)> {
        match self {
            Calc::Yet { items: _, limit: _ } => None,
            Calc::Max(x, y) => Some((x, y))
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Index {
    Left(usize),
    Right(usize),
}

/// Solves the 0-1 knapsack problem.
pub fn knapsack01<T: Item>(items: &[T], limit: u64) -> u64 {
    let mut calcs: Vec<(Index, Calc<T>)> = Vec::new();
    calcs.push((Index::Left(0), Calc::yet(items, limit)));

    loop {
        if let Some((result, calc)) = calcs.pop() {
            match calc {
                Calc::Max(x, y) => {
                    if calcs.is_empty() {
                        break max(x, y);
                    } else {
                        match result {
                            Index::Left(i) => {
                                *calcs[i].1.as_max_mut().unwrap().0 += max(x, y);
                            }
                            Index::Right(i) => {
                                *calcs[i].1.as_max_mut().unwrap().1 += max(x, y);
                            }
                        }
                    }
                }
                Calc::Yet { items, limit } => {
                    if let Some((job, jobs)) = items.split_last() {
                        if job.weight() > limit {
                            calcs.push((result, Calc::yet(jobs, limit)));
                        } else {
                            let i = calcs.len();
                            calcs.push((result, Calc::Max(0, job.value())));
                            calcs.push((Index::Left(i), Calc::yet(jobs, limit)));
                            calcs.push((Index::Right(i), Calc::yet(jobs, limit - job.weight())));
                        }
                    } else {
                        if calcs.is_empty() {
                            break 0;
                        }
                    }
                }
            }
        } else {
            panic!("unreachable");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::max;

    use super::{Item, knapsack01};

    fn knapsack01_rec<T: Item>(items: &[T], limit: u64) -> u64 {
        if let Some((item, items)) = items.split_last() {
            if item.weight() > limit {
                knapsack01_rec(items, limit)
            } else {
                max(knapsack01_rec(items, limit), knapsack01_rec(items, limit - item.weight()) + item.value())
            }
        } else {
            0
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Default, Hash, Debug)]
    struct Bag {
        value: u64,
        weight: u64,
    }

    impl Bag {
        #[inline]
        fn new(value: u64, weight: u64) -> Bag {
            Bag { value: value, weight: weight }
        }
    }

    impl Item for Bag {
        #[inline]
        fn value(&self) -> u64 {
            self.value
        }

        #[inline]
        fn weight(&self) -> u64 {
            self.weight
        }
    }

    #[test]
    fn test_knapsack_rec() {
        let bags = [Bag::new(3, 2), Bag::new(2, 1), Bag::new(2, 1), Bag::new(5, 2)];
        let limit = 4;
        assert_eq!(knapsack01_rec(&bags, limit), 9);

        let bags = [Bag::new(2, 1), Bag::new(5, 2)];
        let limit = 4;
        assert_eq!(knapsack01(&bags, limit), 7);

        let bags = [Bag::new(3, 2), Bag::new(2, 1), Bag::new(2, 1), Bag::new(5, 2)];
        let limit = 4;
        assert_eq!(knapsack01(&bags, limit), 9);
    }

    #[test]
    fn test_knapsack() {
        let bags = [Bag::new(3, 2)];
        let limit = 1;
        assert_eq!(knapsack01(&bags, limit), 0);

        let bags = [Bag::new(2, 1), Bag::new(5, 2)];
        let limit = 4;
        assert_eq!(knapsack01(&bags, limit), 7);

        let bags = [Bag::new(3, 2), Bag::new(2, 1), Bag::new(2, 1), Bag::new(5, 2)];
        let limit = 4;
        assert_eq!(knapsack01(&bags, limit), 9);
    }
}
