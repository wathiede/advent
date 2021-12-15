use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::Infallible,
    fmt::{Debug, Error, Formatter},
    num::ParseIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use thiserror::Error;

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<usize>,
}

impl Image {
    fn new(width: usize, height: usize) -> Image {
        let pixels = vec![0; width * height];
        Image {
            width,
            height,
            pixels,
        }
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f)?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for Image {
    type Output = usize;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[x + y * self.width]
    }
}

impl FromStr for Image {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<_> = s.lines().collect();
        let width = rows[0].len();
        let height = rows.len();
        let pixels = rows
            .iter()
            .flat_map(|row| row.as_bytes().iter().map(|b| (b - b'0') as usize))
            .collect();

        Ok(Image {
            width,
            height,
            pixels,
        })
    }
}

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
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

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}
impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Edge{{node: {}, cost: {}}}", self.node, self.cost)?;
        Ok(())
    }
}

// From https://doc.rust-lang.org/std/collections/binary_heap/index.html

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn make_graph(im: &Image) -> Vec<Vec<Edge>> {
    let idx = |x, y| y * im.width + x;
    let mut graph: Vec<_> = Vec::new();
    for y in 0..im.height {
        for x in 0..im.width {
            let mut edges = Vec::new();
            if x > 0 {
                edges.push(Edge {
                    node: idx(x - 1, y),
                    cost: im[(x - 1, y)],
                });
            }
            if x < im.width - 1 {
                edges.push(Edge {
                    node: idx(x + 1, y),
                    cost: im[(x + 1, y)],
                });
            }
            if y > 0 {
                edges.push(Edge {
                    node: idx(x, y - 1),
                    cost: im[(x, y - 1)],
                });
            }
            if y < im.height - 1 {
                edges.push(Edge {
                    node: idx(x, y + 1),
                    cost: im[(x, y + 1)],
                });
            }
            graph.push(edges);
        }
    }
    graph
}

#[aoc(day15, part1)]
fn part1(input: &str) -> Result<usize> {
    let im: Image = input.parse()?;
    let graph = make_graph(&im);
    Ok(shortest_path(&graph, 0, im.pixels.len() - 1).unwrap())
}

fn x5(im: &Image) -> Image {
    let mut im5 = Image::new(im.width * 5, im.height * 5);
    for iy in 0..5 {
        for ix in 0..5 {
            for y in 0..im.height {
                for x in 0..im.width {
                    let v = im[(x, y)] + ix + iy;
                    let dst_x = ix * im.width + x;
                    let dst_y = iy * im.height + y;
                    im5[(dst_x, dst_y)] = if v > 9 { v % 9 } else { v };
                }
            }
        }
    }
    im5
}

#[aoc(day15, part2)]
fn part2(input: &str) -> Result<usize> {
    let im: Image = input.parse()?;
    let im = x5(&im);
    let graph = make_graph(&im);
    Ok(shortest_path(&graph, 0, im.pixels.len() - 1).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#
        .trim();
        assert_eq!(part1(input)?, 40);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
    "#
        .trim();
        assert_eq!(part2(input)?, 315);
        Ok(())
    }
}
