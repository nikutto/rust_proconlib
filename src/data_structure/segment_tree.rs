pub struct SegmentTree<M, BinOp> {
    n_org: usize,
    n: usize,
    op: BinOp,
    e: M,
    dat: Vec<M>,
}

impl<M, BinOp> SegmentTree<M, BinOp>
where
    BinOp: Fn(M, M) -> M,
    M: Copy,
{
    pub fn new(n: usize, bin_op: BinOp, e: M) -> Self {
        let mut seg_n = 1;
        while seg_n < n {
            seg_n *= 2;
        }
        SegmentTree {
            n_org: n,
            n: seg_n,
            op: bin_op,
            e: e,
            dat: vec![e; seg_n * 2 - 1],
        }
    }

    pub fn update(&mut self, mut pos: usize, x: M) {
        assert!((0..self.n_org).contains(&pos));
        pos += self.n - 1;
        self.dat[pos] = x;
        while pos > 0 {
            let par = (pos - 1) / 2;
            let cl = par * 2 + 1;
            let cr = par * 2 + 2;
            self.dat[par] = (self.op)(self.dat[cl], self.dat[cr]);
            pos = par
        }
    }
    fn query_impl(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> M {
        if a <= l && r <= b {
            self.dat[k]
        } else if r <= a || b <= l {
            self.e
        } else {
            let mid = (l + r) / 2;
            let vl = self.query_impl(a, b, k * 2 + 1, l, mid);
            let vr = self.query_impl(a, b, k * 2 + 2, mid, r);
            (self.op)(vl, vr)
        }
    }
    pub fn query(&self, a: usize, b: usize) -> M {
        assert!(a <= b);
        assert!((0..self.n_org + 1).contains(&a));
        assert!((0..self.n_org + 1).contains(&b));
        self.query_impl(a, b, 0, 0, self.n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn random_pair(md: usize) -> (usize, usize) {
        let l = rand::random::<usize>() % md;
        let r = rand::random::<usize>() % md;
        if l < r {
            (l, r + 1)
        } else {
            (r, l + 1)
        }
    }

    fn test_i64_fold_test<BinOp: Fn(i64, i64) -> i64 + Copy>(n: usize, op: BinOp, e: i64) {
        let op_tmp = op;
        let mut seg = SegmentTree::new(n, op, e);
        let op = op_tmp;
        let vec: Vec<i64> = (0..n)
            .map(|_| rand::random::<i64>() % 10000000007)
            .collect();
        for i in 0..n {
            seg.update(i, vec[i]);
        }
        let t = 25;
        for _ in 0..t {
            let (l, r) = random_pair(n);
            let mut true_v = e;
            for i in l..r {
                true_v = (op)(true_v, vec[i]);
            }
            assert_eq!(true_v, seg.query(l, r));
        }
    }

    fn test_basic_rmq(n: usize) {
        test_i64_fold_test(n, std::cmp::min::<i64>, std::i64::MAX);
    }
    fn test_basic_rsq(n: usize) {
        test_i64_fold_test(n, std::ops::Add::<i64>::add, 0);
    }

    #[test]
    fn test_rmq_25() {
        test_basic_rmq(25);
    }
    #[test]
    fn test_rmq_32() {
        test_basic_rmq(32);
    }

    #[test]
    fn test_rsq_25() {
        test_basic_rsq(25);
    }
    #[test]
    fn test_rsq_32() {
        test_basic_rsq(32);
    }

    #[test]
    fn test_return_e() {
        let n = 25;
        let mut seg = SegmentTree::new(n, std::cmp::min::<i64>, std::i64::MAX);
        let vec: Vec<i64> = (0..n).map(|_| rand::random()).collect();
        for i in 0..n {
            seg.update(i, vec[i]);
        }
        assert_eq!(seg.query(3, 3), std::i64::MAX);
    }

    #[test]
    #[should_panic]
    fn test_out_of_range_update() {
        let n = 25;
        let mut seg = SegmentTree::new(n, std::cmp::min::<i64>, std::i64::MAX);
        seg.update(n, 0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_query_too_large() {
        let n = 25;
        let seg = SegmentTree::new(n, std::cmp::min::<i64>, std::i64::MAX);
        seg.query(n, n + 1);
    }

    #[test]
    #[should_panic]
    fn test_invalid_query_swapped() {
        let n = 25;
        let seg = SegmentTree::new(n, std::cmp::min::<i64>, std::i64::MAX);
        seg.query(5, 2);
    }
}
