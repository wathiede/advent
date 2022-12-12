use std::{cmp::Ordering, collections::BinaryHeap};

use advent::prelude::*;
use aoc_runner_derive::aoc;

#[derive(Clone, Debug)]
struct Grid {
    cells: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    position: (usize, usize),
}
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: (usize, usize),
    cost: isize,
}

impl Index<(usize, usize)> for Grid {
    type Output = usize;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y][x]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[y][x]
    }
}

impl Grid {
    fn edges(&self, pos: (usize, usize)) -> Vec<Edge> {
        // Only return edges that are 1 or 0 difference in height.
        // All costs are equal.
        let mut edges = Vec::new();
        let h = self[pos] as isize;
        let (x, y) = pos;
        if x > 0 {
            let node = (x - 1, y);
            let cost = self[node] as isize - h;
            if cost <= 1 {
                edges.push(Edge { node, cost: 1 });
            }
        }
        if x < self.width - 1 {
            let node = (x + 1, y);
            let cost = self[node] as isize - h;
            if cost <= 1 {
                edges.push(Edge { node, cost: 1 });
            }
        }
        if y > 0 {
            let node = (x, y - 1);
            let cost = self[node] as isize - h;
            if cost <= 1 {
                edges.push(Edge { node, cost: 1 });
            }
        }
        if y < self.height - 1 {
            let node = (x, y + 1);
            let cost = self[node] as isize - h;
            if cost <= 1 {
                edges.push(Edge { node, cost: 1 });
            }
        }
        edges
    }
    // From https://doc.rust-lang.org/std/collections/binary_heap/index.html

    // Dijkstra's shortest path algorithm.

    // Start at `start` and use `dist` to track the current shortest distance
    // to each node. This implementation isn't memory-efficient as it may leave duplicate
    // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
    // for a simpler implementation.
    fn shortest_path(&self) -> Option<isize> {
        // dist[node] = current shortest distance from `start` to `node`
        let mut dist: HashMap<(usize, usize), isize> = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| ((x, y), isize::MAX)))
            .collect();

        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost
        dist.insert(self.start, 0);
        heap.push(State {
            cost: 0,
            position: self.start,
        });

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, position }) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == self.end {
                return Some(cost);
            }

            // Important as we may have already found a better way
            if cost > dist[&position] {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for edge in self.edges(position) {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                // If so, add it to the frontier and continue
                //dbg!(&dist, &next.position);
                if next.cost < dist[&next.position] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist.insert(next.position, next.cost);
                }
            }
        }

        // Goal not reachable
        None
    }
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let cells: Vec<Vec<_>> = s
            .lines()
            .enumerate()
            .map(|(y, r)| {
                r.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(x, b)| {
                        match b {
                            b'S' => {
                                start = (x, y);
                                0
                            }
                            b'E' => {
                                end = (x, y);
                                b'z' - b'a'
                            }
                            b => b - b'a',
                        }
                        .into()
                    })
                    .collect()
            })
            .collect();
        let height = cells.len();
        let width = cells[0].len();
        Ok(Grid {
            cells,
            width,
            height,
            start,
            end,
        })
    }
}

#[aoc(day12, part1)]
fn part1(input: &str) -> isize {
    let g: Grid = input.parse().expect("grid");
    //dbg!(&g);
    g.shortest_path().expect("failed to find path")
}

#[aoc(day12, part2)]
fn part2(input: &str) -> isize {
    let mut g: Grid = input.parse().expect("grid");
    //dbg!(&g);
    let width = g.width;
    let height = g.height;
    let starts: Vec<_> = (0..height)
        .flat_map(|y| {
            let g = &g;
            (0..width).filter_map(move |x| {
                if g.cells[y][x] == 0 {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect();
    starts
        .iter()
        .filter_map(|(x, y)| {
            g.start = (*x, *y);
            g.shortest_path()
        })
        .min()
        .expect("couldn't find min")
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 31);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 29);
    }
}
