// Bellman-Ford style DP with a stop limit
//
// k stops means we can use at most k + 1 flights/edges.
// Each loop iteration allows paths to use one more flight.
//
// dist[v] = cheapest cost to reach city v using at most the flights allowed
// from previous iteration.
//
// In each round, clone dist into next:
// * read from dist = previous round
// * write into next = current round
//
// This is important because the stop limit matters. If we updated dist in-place,
// one iteration could chain multiple flights and use more then one new edge.

// Time: O((K + 1) * E), where E = flights.len().
// Space: O(V), where V = number of cities.

pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n = n as usize;
    let src = src as usize;
    let dst = dst as usize;

    // Use i32::MAX / 2 to avoid overflow when adding a flight cost.
    let inf = i32::MAX / 2;
    let mut dist = vec![inf; n];
    dist[src] = 0;

    for _ in 0..=k {
        let mut next = dist.clone();

        for flight in &flights {
            let u = flight[0] as usize;
            let v = flight[1] as usize;
            let w = flight[2];

            if dist[u] != inf {
                next[v] = next[v].min(dist[u] + w);
            }
        }

        dist = next;
    }

    if dist[dst] == inf { -1 } else { dist[dst] }
}
