// The problem here is making use of graph theory properties telling that:
// * Origin graph was a tree meaning: One group with no cycles and edges.len() == n - 1;
// * One edge is added making a cycle there and making n = edges.len()

// To resolve the problem we need to find the last edge which make a cycle in the tree.
// It's possible to do it with cycle detection with DFS but it will not be as efficient as
// Disjoint Set Unions (Union Find) as we just need to find the first union call which
// fails because both endpoints are already in the same component.

// Time: O((V + E) * alpha(V)), effectively near O(V + E), where alpha is the inverse
// Ackermann function.
// Time: O(E * α(V)), effectively near O(E) as E == V for this problem;
// Space O(V) for parent and size arrays.

struct DisjointUnion {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointUnion {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];

        Self { parent, size }
    }

    fn find_parent(&mut self, node: usize) -> usize {
        let Self { parent, .. } = self;

        let mut cur = node;
        while cur != parent[cur] {
            parent[cur] = parent[parent[cur]];
            cur = parent[cur];
        }

        cur
    }

    fn union(&mut self, node1: usize, node2: usize) -> bool {
        let par1 = self.find_parent(node1);
        let par2 = self.find_parent(node2);
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

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    // In valid tree `edges.len() == n - 1` which make :
    // `n = edges.len()` since the graph has one redundant edge.

    let n = edges.len();
    let mut dsu = DisjointUnion::new(n);
    for edge in edges {
        // edges starts with 1 to n
        if !dsu.union(edge[0] as usize - 1, edge[1] as usize - 1) {
            return edge;
        }
    }

    panic!("Problem description guarantees having answer");
}
