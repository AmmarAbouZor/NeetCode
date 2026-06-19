// Union Find (Disjoint Set Union) with path compression and union by size.
//
// Time: O((V + E) * α(V)), effectively near O(V + E), where α is the
// inverse Ackermann function.
// Space: O(V), for parent and size arrays.

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];

        Self { parent, size }
    }

    fn find(&mut self, node: usize) -> usize {
        let Self { parent, .. } = self;
        let mut cur = node;
        while cur != parent[cur] {
            // Path compression: Update parent for all nodes in the branch
            // to point to top parent directly.
            parent[cur] = parent[parent[cur]];
            cur = parent[cur];
        }
        cur
    }

    fn union(&mut self, node1: usize, node2: usize) -> bool {
        let par1 = self.find(node1);
        let par2 = self.find(node2);

        // Check if nodes already in same group.
        if par1 == par2 {
            return false;
        }

        let Self { parent, size } = self;

        if size[par1] > size[par2] {
            parent[par2] = par1;
            size[par1] += size[par2];
        } else {
            parent[par1] = par2;
            size[par2] += size[par1];
        }

        true
    }
}

pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut dsu = Dsu::new(n);

    let mut components = n;

    for pair in edges {
        if dsu.union(pair[0] as usize, pair[1] as usize) {
            components -= 1;
        }
    }

    components as i32
}
