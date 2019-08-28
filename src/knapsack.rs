use std::cmp::max;

pub trait Item {
    fn value(&self) -> u64;
    fn weight(&self) -> u64;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Calc<'a, T> {
    Yet { items: &'a [T], limit: u64 },
    Max(u64, u64),
}

impl<'a, T> Calc<'a, T> {
    fn yet(items: &'a [T], limit: u64) -> Calc<'a, T> {
        Calc::Yet { items: items, limit: limit }
    }
}

pub fn knapsack01<T: Item>(items: &[T], limit: u64) -> u64 {
    unsafe {
        let mut result = 0;
        let mut calcs: Vec<Box<(*mut u64, Calc<T>)>> = Vec::new();
        calcs.push(Box::new((&mut result, Calc::yet(items, limit))));

        loop {
            if let Some((result, calc)) = calcs.pop().map(|x| *x) {
                match calc {
                    Calc::Max(x, y) => { *result += max(x, y) }
                    Calc::Yet { items, limit } => {
                        if let Some((job, jobs)) = items.split_last() {
                            if job.weight() > limit {
                                calcs.push(Box::new((result, Calc::yet(jobs, limit))));
                            } else {
                                calcs.push(Box::new((result, Calc::Max(0, job.value()))));
                                let (res0, res1): (*mut u64, *mut u64) = {
                                    if let &mut (_, Calc::Max(ref mut res0, ref mut res1)) = calcs.last_mut().unwrap().as_mut() {
                                        (res0, res1)
                                    } else {
                                        panic!("unreachable");
                                    }
                                };
                                calcs.push(Box::new((res0, Calc::yet(jobs, limit))));
                                calcs.push(Box::new((res1, Calc::yet(jobs, limit - job.weight()))));
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        result
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
