use cargo_snippet::snippet;

#[snippet("union_find")]
struct UnionFind {
    pub parent: Vec<usize>,
    pub sum: Vec<usize>,
}

#[snippet("union_find")]
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        return UnionFind {
            parent: (0..n).collect(),
            sum: vec![1; n],
        };
    }
    fn root(self: &mut UnionFind, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        } else {
            self.parent[x] = self.root(self.parent[x]);
            return self.parent[x];
        }
    }
    pub fn unite(self: &mut UnionFind, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        let sum = self.sum[x] + self.sum[y];
        self.parent[x] = y;
        self.sum[x] = sum;
        self.sum[y] = sum;
    }
    pub fn is_equivalent(self: &mut UnionFind, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
    pub fn group_size(self: &UnionFind, x: usize) -> usize {
        return self.sum[x];
    }
}
