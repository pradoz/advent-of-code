use ::std::cmp::Ordering;
use aoc_utils::parse_lines;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq)]
struct OrderedFloat(f32);

impl OrderedFloat {
    fn new(val: f32) -> Self {
        OrderedFloat(val)
    }
}

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            return None;
        }

        let x: u32 = parts[0].parse().ok()?;
        let y: u32 = parts[1].parse().ok()?;
        let z: u32 = parts[2].parse().ok()?;

        Some(Self { x, y, z })
    }

    fn euclidean_distance(&self, other: &Point) -> f32 {
        fn _sub(a: u32, b: u32) -> f32 {
            a as f32 - b as f32
        }

        let x = _sub(self.x, other.x).powi(2);
        let y = _sub(self.y, other.y).powi(2);
        let z = _sub(self.z, other.z).powi(2);
        (x + y + z).sqrt()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Edge {
    u: usize,
    v: usize,
    weight: OrderedFloat,
}

impl Edge {
    fn new(u: usize, v: usize, points: &[Point]) -> Self {
        let p1 = &points[u];
        let p2 = &points[v];
        let weight = OrderedFloat::new(p1.euclidean_distance(p2));
        Self { u, v, weight }
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse for min-heap (BinaryHeap is max-heap by default)
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_edges(points: &Vec<Point>) -> BinaryHeap<Edge> {
    let mut heap = BinaryHeap::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let edge = Edge::new(i, j, points);
            heap.push(edge);
        }
    }

    heap
}

#[derive(Debug, Clone, PartialEq)]
struct UnionFind {
    parent: Vec<usize>,
    set_size: Vec<usize>,
    n: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent: Vec<usize> = Vec::with_capacity(n);
        let mut set_size: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            parent.push(i);
            set_size.push(1);
        }

        Self {
            parent,
            set_size,
            n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, u: usize, v: usize) -> bool {
        let root_u = self.find(u);
        let root_v = self.find(v);

        // in the same set, no merge needed
        if root_u == root_v {
            return false;
        }

        // union by size. attach smaller tree to larger tree
        if self.set_size[root_u] < self.set_size[root_v] {
            self.parent[root_u] = root_v;
            self.set_size[root_v] += self.set_size[root_u];
        } else {
            self.parent[root_v] = root_u;
            self.set_size[root_u] += self.set_size[root_v];
        }

        true // we merged
    }

    fn is_connected(&mut self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }

    fn component_sizes(&self) -> Vec<usize> {
        let mut sizes: HashMap<usize, usize> = HashMap::with_capacity(self.n);

        for i in 0..self.n {
            if self.parent[i] == i {
                // i is a root
                sizes.insert(i, self.set_size[i]);
            }
            // let root = self.find(i);
            // *sizes.entry(root).or_insert(0) += 1;
        }

        sizes.values().copied().collect()
    }

    fn num_components(&self) -> usize {
        (0..self.n).filter(|&i| self.parent[i] == i).count()
    }
}

fn part1(points: &Vec<Point>, k: usize) -> usize {
    let n = points.len();
    let mut edges = generate_edges(points);
    let mut uf = UnionFind::new(n);

    for _ in 0..k {
        if edges.is_empty() {
            break;
        }

        let edge = edges.pop().unwrap();
        uf.union(edge.u, edge.v);
    }

    let mut comp_sizes = uf.component_sizes();
    comp_sizes.sort_by(|a, b| b.cmp(a));

    comp_sizes.iter().take(3).product()
}

fn part2(points: &Vec<Point>) -> usize {
    let n = points.len();
    let mut edges = generate_edges(points);
    let mut uf = UnionFind::new(n);

    let mut connections = 0;

    while !edges.is_empty() && connections < n - 1 {
        let edge = edges.pop().unwrap();

        if uf.union(edge.u, edge.v) {
            connections += 1
        }

        if connections == n - 1 {
            let u = points[edge.u];
            let v = points[edge.v];
            return (u.x  as usize) * (v.x as usize);
        }
    }
    0
}

fn main() -> std::io::Result<()> {
    let points: Vec<Point> = parse_lines("2025/day8/input.txt", Point::parse)?;

    println!("Part 1: {}", part1(&points, 1000));
    println!("Part 2: {}", part2(&points));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_FULL: &str = "\
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689";

    fn parse_test_input() -> Vec<Point> {
        TEST_INPUT_FULL
            .lines()
            .filter_map(|line| Point::parse(line))
            .collect()
    }

    #[test]
    fn test_parse() {
        let p = parse_test_input();
        assert_eq!(p.len(), 20);
        assert_eq!(p[0], Point::new(162, 817, 812));
        assert_eq!(p[2], Point::new(906, 360, 560));
        assert_eq!(p[12], Point::new(117, 168, 530));
        assert_eq!(p[19], Point::new(425, 690, 689));
    }

    #[test]
    fn test_edge() {
        let u = Point::new(1, 2, 3);
        let v = Point::new(4, 5, 6);
        let points = vec![u, v];
        let e = Edge::new(0, 1, &points);
        assert_eq!(e.u, 0);
        assert_eq!(e.v, 1);
        assert_eq!(e.weight, OrderedFloat::new((27 as f32).sqrt()));
    }

    #[test]
    fn test_init_parent_points_to_self() {
        let uf = UnionFind::new(5);
        assert_eq!(uf.n, 5);
        assert_eq!(uf.parent.len(), 5);
        assert_eq!(uf.set_size.len(), 5);

        for i in 0..5 {
            assert_eq!(uf.parent[i], i);
            assert_eq!(uf.set_size[i], 1);
        }
    }

    #[test]
    fn test_init_find_self() {
        let mut uf = UnionFind::new(5);
        for i in 0..5 {
            assert_eq!(uf.find(i), i);
        }
    }

    #[test]
    fn test_union_merge_same_root() {
        let mut uf = UnionFind::new(5);
        assert!(uf.union(0, 1));
        assert_eq!(uf.find(0), uf.find(1));
    }

    #[test]
    fn test_union_same_set() {
        let mut uf = UnionFind::new(5);

        uf.union(0, 1);

        assert!(!uf.union(0, 1));
        assert!(!uf.union(1, 0));
    }

    #[test]
    fn test_union_transitive() {
        let mut uf = UnionFind::new(5);

        uf.union(0, 1);
        uf.union(1, 2);

        let root = uf.find(0);
        assert_eq!(uf.find(1), root);
        assert_eq!(uf.find(2), root);
    }

    #[test]
    fn test_union_by_size() {
        let mut uf = UnionFind::new(5);

        uf.union(0, 1);
        uf.union(0, 2);

        uf.union(3, 4);

        uf.union(0, 3);

        // all should be in the same set
        let root = uf.find(0);
        for i in 1..5 {
            assert_eq!(uf.find(i), root);
        }
    }

    #[test]
    fn test_path_compression() {
        let mut uf = UnionFind::new(5);

        // 0->1->2->3
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);

        let root = uf.find(0); // path compression should happen

        // all elements should point directly to root
        uf.find(0);
        assert_eq!(uf.parent[0], root);
    }

    #[test]
    fn test_is_connected() {
        let mut uf = UnionFind::new(5);

        assert!(!uf.is_connected(0, 1));

        uf.union(0, 1);

        assert!(uf.is_connected(0, 1));
        assert!(uf.is_connected(1, 0)); // symmetric

        assert!(!uf.is_connected(0, 2));
    }

    #[test]
    fn test_num_components() {
        let mut uf = UnionFind::new(5);

        // start with 5 separate components
        assert_eq!(uf.num_components(), 5);

        uf.union(0, 1);
        assert_eq!(uf.num_components(), 4);

        uf.union(2, 3);
        assert_eq!(uf.num_components(), 3);

        uf.union(0, 2);
        assert_eq!(uf.num_components(), 2);

        uf.union(1, 4);
        assert_eq!(uf.num_components(), 1);
    }

    #[test]
    fn test_init_component_sizes() {
        let uf = UnionFind::new(5);

        let sizes = uf.component_sizes();
        assert_eq!(sizes.len(), 5);

        for &size in &sizes {
            assert_eq!(size, 1);
        }
    }

    #[test]
    fn test_component_sizes_after_unions() {
        let mut uf = UnionFind::new(5);

        uf.union(0, 1);
        uf.union(2, 3);

        let mut sizes = uf.component_sizes();
        sizes.sort();

        // Should have 3 components: size 1, 2, 2
        assert_eq!(sizes, vec![1, 2, 2]);
    }

    #[test]
    fn test_component_sizes_single_component() {
        let mut uf = UnionFind::new(5);

        // union all into a single component
        for i in 1..5 {
            uf.union(0, i);
        }

        let sizes = uf.component_sizes();
        assert_eq!(sizes.len(), 1);
        assert_eq!(sizes[0], 5);
    }

    #[test]
    fn test_multiple_unions_same_elements() {
        let mut uf = UnionFind::new(5);

        assert!(uf.union(0, 1));
        assert!(!uf.union(0, 1)); // already connected
        assert!(!uf.union(1, 0)); // already connected

        assert_eq!(uf.num_components(), 4);
    }

    #[test]
    fn test_complex_scenario() {
        let mut uf = UnionFind::new(10);

        // 0->1->2->3
        // 4->5
        // 6->7->8
        // 9
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);

        uf.union(4, 5);

        uf.union(6, 7);
        uf.union(7, 8);

        assert_eq!(uf.num_components(), 4);

        let mut sizes = uf.component_sizes();
        sizes.sort();
        assert_eq!(sizes, vec![1, 2, 3, 4]);

        assert!(uf.is_connected(0, 3));
        assert!(uf.is_connected(4, 5));
        assert!(uf.is_connected(6, 8));
        assert!(!uf.is_connected(0, 4));
        assert!(!uf.is_connected(4, 6));
    }

    #[test]
    fn test_empty_union_find() {
        let uf = UnionFind::new(0);
        assert_eq!(uf.num_components(), 0);
        assert_eq!(uf.component_sizes().len(), 0);
    }

    #[test]
    fn test_single_element() {
        let mut uf = UnionFind::new(1);
        assert_eq!(uf.num_components(), 1);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.component_sizes(), vec![1]);
    }

    #[test]
    fn test_union_chain() {
        let mut uf = UnionFind::new(100);

        // union in a chain
        for i in 0..99 {
            uf.union(i, i + 1);
        }

        assert_eq!(uf.num_components(), 1);
        assert_eq!(uf.component_sizes(), vec![100]);

        // all should have the same root
        let root = uf.find(0);
        for i in 1..100 {
            assert_eq!(uf.find(i), root);
        }
    }

    #[test]
    fn test_disjoint_pairs() {
        let mut uf = UnionFind::new(10);

        // 5 disjoint pairs
        for i in (0..10).step_by(2) {
            uf.union(i, i + 1);
        }

        assert_eq!(uf.num_components(), 5);

        let sizes = uf.component_sizes();
        assert_eq!(sizes.len(), 5);
        for &size in &sizes {
            assert_eq!(size, 2);
        }
    }

    #[test]
    fn test_set_size_accuracy() {
        let mut uf = UnionFind::new(6);

        uf.union(0, 1);
        let root1 = uf.find(0);
        assert_eq!(uf.set_size[root1], 2);

        uf.union(2, 3);
        let root2 = uf.find(2);
        assert_eq!(uf.set_size[root2], 2);

        uf.union(0, 2); // merge two size-2 sets
        let root_final = uf.find(0);
        assert_eq!(uf.set_size[root_final], 4);
    }

    #[test]
    fn test_part1() {
        let points = parse_test_input();
        assert_eq!(part1(&points, 10), 40);
    }

    #[test]
    fn test_part2() {
        let points = parse_test_input();
        assert_eq!(part2(&points), 25272);
    }
}
