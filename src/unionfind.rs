pub struct UnionFind {
    // A positive number represents the parent node and a negative number represents the size of
    // the containing set.
    parents: Vec<isize>,
}

impl UnionFind {
    #[inline]
    pub fn new(n: usize) -> Self {
        let parents = vec![-1; n];
        UnionFind { parents }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.parents.len()
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parents[i] < 0 {
            i
        } else {
            let r = self.parents[i] as usize;
            self.parents[i] = self.find(r) as isize;
            self.parents[i] as usize
        }
    }

    pub fn unite(&mut self, i: usize, j: usize) {
        let mut i = self.find(i);
        let mut j = self.find(j);
        if self.parents[i] > self.parents[j] {
            std::mem::swap(&mut i, &mut j);
        }
        self.parents[i] += self.parents[j];
        self.parents[j] = i as isize;
    }

    #[inline]
    pub fn are_same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    #[inline]
    pub fn size(&mut self, i: usize) -> usize {
        let r = self.find(i);
        -self.parents[r] as usize
    }
}
