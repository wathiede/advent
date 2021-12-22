use advent::prelude::*;
use aoc_runner_derive::aoc;
use std::{
    io::{BufReader, Cursor, Read},
    ops::Add,
};

#[derive(Copy, Clone, Debug, PartialEq)]
enum ChildType {
    None,
    Value(usize),
    Subtree(Idx),
}

#[derive(Copy, Clone, Default, PartialEq)]
struct Idx(usize);

impl Debug for Idx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Idx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    deleted: bool,
    idx: Idx,
    parent: Option<Idx>,
    left: ChildType,
    right: ChildType,
}
// Tree needs to support merging two into one for adding snailfish numbers.
// Tree needs to support rightward and leftward depth first searches to find neighbors for applying
// exploded spill over.
// Need to support remove and/or replace for explode.
// Need to support insert and/or replace for split.
#[derive(Debug, Default)]
struct Tree {
    root: Idx,
    nodes: Vec<Node>,
}

struct TreeIter {
    it: std::vec::IntoIter<Idx>,
}

impl TreeIter {
    fn new(indices: &[Idx]) -> TreeIter {
        let indices = indices.to_vec();
        TreeIter {
            it: indices.into_iter(),
        }
    }
}

impl Iterator for TreeIter {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        // Lazy but should work.
        self.to_string() == other.to_string()
    }
}

fn read_byte<R: Read>(reader: &mut R) -> std::io::Result<Option<u8>> {
    reader.bytes().next().transpose()
}

impl Tree {
    fn reduce(&mut self) {
        let mut changed = true;
        while changed {
            changed = self.explode();
            println!("after explode {}", self);
            if changed {
                continue;
            }
            changed = self.split();
            println!("after split {}", self);
            //println!("splice changed {}", changed);
        }
    }
    fn magnitude(&self) -> usize {
        fn inner(tree: &Tree, node: &Node) -> usize {
            match (node.left, node.right) {
                (ChildType::Value(l), ChildType::Value(r)) => 3 * l + 2 * r,
                (ChildType::Subtree(idx), ChildType::Value(r)) => {
                    3 * inner(&tree, &tree[idx]) + 2 * r
                }
                (ChildType::Value(l), ChildType::Subtree(idx)) => {
                    3 * l + 2 * inner(&tree, &tree[idx])
                }
                (ChildType::Subtree(l_idx), ChildType::Subtree(r_idx)) => {
                    3 * inner(&tree, &tree[l_idx]) + 2 * inner(&tree, &tree[r_idx])
                }
                _ => panic!("unhandled combo for magnitude"),
            }
        }
        inner(self, &self[self.root])
    }

    fn split(&mut self) -> bool {
        if let Some(split_idx) = self
            .left_to_right()
            .skip_while(|idx| {
                let n = &self[*idx];
                if let ChildType::Value(v) = n.left {
                    if v > 9 {
                        return false;
                    }
                }
                if let ChildType::Value(v) = n.right {
                    if v > 9 {
                        return false;
                    }
                }
                true
            })
            .next()
        {
            if let ChildType::Value(v) = self[split_idx].left {
                if v > 9 {
                    let l = v / 2;
                    let r = if v % 2 == 1 { 1 + v / 2 } else { v / 2 };
                    let mut new_idx = self.add_node(ChildType::Value(l), ChildType::Value(r));
                    self[new_idx].parent = Some(split_idx);
                    self[split_idx].left = ChildType::Subtree(new_idx);
                }
            }
            if let ChildType::Value(v) = self[split_idx].right {
                if v > 9 {
                    let l = v / 2;
                    let r = if v % 2 == 1 { 1 + v / 2 } else { v / 2 };
                    let mut new_idx = self.add_node(ChildType::Value(l), ChildType::Value(r));
                    self[new_idx].parent = Some(split_idx);
                    self[split_idx].right = ChildType::Subtree(new_idx);
                }
            }
            return true;
        }
        false
    }
    fn explode(&mut self) -> bool {
        let mut changed = false;
        if let Some(node) = self
            .nodes
            .iter()
            .filter(|n| !n.deleted)
            .find(|n| self.depth(n) >= 4)
        {
            changed = true;
            let ex_idx = node.idx;
            // Find spillover to the right
            if let Some(spillover) = self
                .left_to_right()
                .skip_while(|idx| *idx != ex_idx)
                .skip(1)
                .find(|idx| {
                    let n = &self[*idx];
                    match (n.left, n.right) {
                        (ChildType::Subtree(_), ChildType::Subtree(_)) => false,
                        _ => true,
                    }
                })
            {
                let src = self[ex_idx].right;
                let tgt = &mut self[spillover];
                if let (ChildType::Value(l), ChildType::Value(r)) = (src, tgt.left) {
                    tgt.left = ChildType::Value(l + r);
                } else if let (ChildType::Value(l), ChildType::Value(r)) = (src, tgt.right) {
                    tgt.right = ChildType::Value(l + r);
                } else {
                    unreachable!()
                };
            }
            // Find spillover to the left
            if let Some(spillover) = self
                .right_to_left()
                .skip_while(|idx| *idx != ex_idx)
                .skip(1)
                .find(|idx| {
                    let n = &self[*idx];
                    match (n.left, n.right) {
                        (ChildType::Subtree(_), ChildType::Subtree(_)) => false,
                        _ => true,
                    }
                })
            {
                let src = self[ex_idx].left;
                let tgt = &mut self[spillover];
                if let (ChildType::Value(l), ChildType::Value(r)) = (src, tgt.right) {
                    tgt.right = ChildType::Value(l + r);
                } else if let (ChildType::Value(l), ChildType::Value(r)) = (src, tgt.left) {
                    tgt.left = ChildType::Value(l + r);
                } else {
                    unreachable!()
                };
            }
            // Replace exploded node
            self[ex_idx].deleted = true;
            let p_idx = self[ex_idx].parent.expect("exploded root");
            let p = &mut self[p_idx];
            if let ChildType::Subtree(idx) = p.left {
                if idx == ex_idx {
                    p.left = ChildType::Value(0);
                }
            }
            if let ChildType::Subtree(idx) = p.right {
                if idx == ex_idx {
                    p.right = ChildType::Value(0);
                }
            }
        }
        changed
    }
    fn depth(&self, node: &Node) -> usize {
        if let Some(parent_idx) = node.parent {
            1 + self.depth(&self[parent_idx])
        } else {
            0
        }
    }
    fn find_root(&self, node: &Node) -> Idx {
        match node.parent {
            Some(parent_idx) => self.find_root(&self[parent_idx]),
            None => node.idx,
        }
    }
    fn add_node(&mut self, left: ChildType, right: ChildType) -> Idx {
        let idx = Idx(self.nodes.len());
        self.nodes.push(Node {
            deleted: false,
            idx,
            parent: None,
            left,
            right,
        });
        idx
    }

    fn from_str_node(&mut self, r: &mut BufReader<Cursor<&[u8]>>) -> ChildType {
        let mut parsing_left = true;
        // Can this be rewritten to eliminate the need for `None`?
        let mut left = ChildType::None;
        let mut right = ChildType::None;
        while let Ok(Some(b)) = read_byte(r) {
            match b {
                b'[' => {
                    let node = self.from_str_node(r);
                    if parsing_left {
                        left = node;
                    } else {
                        right = node;
                    }
                }
                b']' => {
                    let mut left_idx = None;
                    let mut right_idx = None;
                    if let ChildType::Subtree(idx) = left {
                        left_idx = Some(idx);
                    }
                    if let ChildType::Subtree(idx) = right {
                        right_idx = Some(idx);
                    }
                    let child_idx = self.add_node(left, right);
                    if let Some(idx) = left_idx {
                        self[idx].parent = Some(child_idx);
                    }
                    if let Some(idx) = right_idx {
                        self[idx].parent = Some(child_idx);
                    }
                    return ChildType::Subtree(child_idx);
                }
                b',' => parsing_left = false,
                b'0'..=b'9' => {
                    let mut v = b - b'0';
                    if let Ok(Some(peek)) = read_byte(r) {
                        match peek {
                            b'0'..=b'9' => v = (peek - b'0') + 10 * v,
                            // Wasn't a number >9, push the byte back into the buffer.
                            _ => r.seek_relative(-1).expect("failed to seek"),
                        }
                    }
                    if parsing_left {
                        left = ChildType::Value(v.into());
                        parsing_left = false;
                    } else {
                        right = ChildType::Value(v.into());
                    }
                    continue;
                }
                _ => panic!("unknown byte '{}'", b),
            }
        }
        unreachable!()
    }

    fn fmt_node(&self, f: &mut Formatter<'_>, node: &Node) -> std::fmt::Result {
        write!(f, "[")?;
        match node.left {
            ChildType::None => panic!("left node was None"),
            ChildType::Value(v) => write!(f, "{}", v)?,
            ChildType::Subtree(idx) => self.fmt_node(f, &self[idx])?,
        };
        write!(f, ",")?;
        match node.right {
            ChildType::None => panic!("right node was None"),
            ChildType::Value(v) => write!(f, "{}", v)?,
            ChildType::Subtree(idx) => self.fmt_node(f, &self[idx])?,
        };
        write!(f, "]")?;
        Ok(())
    }

    fn left_to_right(&mut self) -> TreeIter {
        fn dfs(tree: &Tree, n: &Node, mut indices: &mut Vec<Idx>) {
            if let ChildType::Subtree(idx) = n.left {
                dfs(tree, &tree[idx], indices);
            }
            indices.push(n.idx);
            if let ChildType::Subtree(idx) = n.right {
                dfs(tree, &tree[idx], indices);
            }
        }
        let mut indices = Vec::with_capacity(self.nodes.len());
        dfs(self, &self[self.root], &mut indices);
        TreeIter::new(&indices)
    }
    fn right_to_left(&mut self) -> TreeIter {
        fn dfs(tree: &Tree, n: &Node, mut indices: &mut Vec<Idx>) {
            if let ChildType::Subtree(idx) = n.right {
                dfs(tree, &tree[idx], indices);
            }
            indices.push(n.idx);
            if let ChildType::Subtree(idx) = n.left {
                dfs(tree, &tree[idx], indices);
            }
        }
        let mut indices = Vec::with_capacity(self.nodes.len());
        dfs(self, &self[self.root], &mut indices);
        TreeIter::new(&indices)
    }
}

impl Add for Tree {
    type Output = Tree;

    fn add(self, other: Self) -> Self {
        // This is lazy but works for simple any obvious reasons (if FromStr and Display work
        // correctly).
        format!("[{},{}]", self, other)
            .parse()
            .expect("failed to parse merge tree")
    }
}

impl FromStr for Tree {
    type Err = Infallible;

    fn from_str(input: &str) -> std::result::Result<Tree, Infallible> {
        let mut tree = Tree::default();
        let mut bytes = input.as_bytes();
        assert_eq!(
            read_byte(&mut bytes).expect("couldn't read first byte"),
            Some(b'[')
        );
        let mut b = BufReader::new(Cursor::new(bytes));
        tree.from_str_node(&mut b);
        tree.root = tree.find_root(&tree[Idx(0)]);
        Ok(tree)
    }
}

impl Index<Idx> for Tree {
    type Output = Node;
    fn index(&self, idx: Idx) -> &Self::Output {
        &self.nodes[idx.0]
    }
}

impl IndexMut<Idx> for Tree {
    fn index_mut(&mut self, idx: Idx) -> &mut Self::Output {
        &mut self.nodes[idx.0]
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.nodes.is_empty() {
            return write!(f, "[]");
        }

        let node = &self[self.root];
        self.fmt_node(f, &node)?;
        Ok(())
    }
}

fn sum(input: &str) -> Tree {
    input
        .lines()
        .map(|l| l.parse().expect("failed to parse"))
        .reduce(|acc, t| acc + t)
        .expect("failed to reduce")
}

#[aoc(day18, part1)]
fn part1(input: &str) -> Result<usize> {
    let nums: Vec<Tree> = input
        .lines()
        .map(|l| {
            dbg!(l);
            l.parse().expect("failed to parse")
        })
        .collect();
    let mut it = nums.into_iter();
    let mut last = it.next().unwrap();
    while let Some(next) = it.next() {
        println!("  {}", last);
        println!("+ {}", next);
        last = last + next;
        println!("= {}", last);
        last.reduce();
        println!("= {}\n", last);
    }
    Ok(last.magnitude())
}

/*
#[aoc(day18, part2)]
fn part2(input: &str) -> Result<usize> {
todo!("part2");
Ok(0)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() -> Result<()> {
        for (i, s) in ["[1,2]", "[[1,2],3]", "[1,[2,3]]", "[[1,2],[3,4]]"]
            .into_iter()
            .enumerate()
        {
            let t = s.parse::<Tree>()?;
            assert_eq!(&t.to_string(), s, "input {}: '{}'", i, s);
            //assert_eq!(&t.to_string(), s, "input {}: '{}'\ntree: {:#?}", i, s, t);
        }
        Ok(())
    }

    #[test]
    fn test_sum() -> Result<()> {
        let l: Tree = "[1,2]".parse().unwrap();
        let r: Tree = "[[3,4],5]".parse().unwrap();
        assert_eq!(l + r, "[[1,2],[[3,4],5]]".parse().unwrap());

        let input = r#"
[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]
    "#
        .trim();
        let mut s = sum(input);
        s.reduce();
        assert_eq!(s.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        Ok(())
    }

    #[test]
    fn test_reduce() -> Result<()> {
        for (input, want) in [
            ("[0,[0,[0,[0,[0,0]]]]]", "[0,[0,[0,[0,0]]]]"),
            ("[[[[[[[[0,0],0],0],0],0],0],0],0]", "[[[[0,0],0],0],0]"),
            ("[[[[[[[0,0],0],0],0],0],0],0]", "[[[[0,0],0],0],0]"),
        ] {
            println!("== test_reduce: {}", input);
            let mut tree: Tree = input.parse()?;
            tree.reduce();
            let want = want.parse()?;
            assert_eq!(tree, want, "\nInput {} Got {} Want {}", input, tree, want);
        }
        Ok(())
    }
    #[test]
    fn test_explode() -> Result<()> {
        for (input, want) in [
            ("[[[[0,0],0],0],0]", "[[[[0,0],0],0],0]"),
            ("[[[0,0],0],0]", "[[[0,0],0],0]"),
            ("[[0,0],0]", "[[0,0],0]"),
            ("[0,[0,[0,[0,0]]]]", "[0,[0,[0,[0,0]]]]"),
            ("[0,[0,[0,0]]]", "[0,[0,[0,0]]]"),
            ("[0,[0,0]]", "[0,[0,0]]"),
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ] {
            println!("== test_explode: {}", input);
            let mut tree: Tree = input.parse()?;
            tree.explode();
            let want = want.parse()?;
            assert_eq!(tree, want, "\nInput {} Got {} Want {}", input, tree, want);
        }
        Ok(())
    }

    #[test]
    fn test_split() -> Result<()> {
        for (input, want) in [
            ("[10,0]", "[[5,5],0]"), //
            ("[0,11]", "[0,[5,6]]"),
            ("[[0,11],0]", "[[0,[5,6]],0]"),
            ("[11,0]", "[[5,6],0]"),
            ("[0,[11,0]]", "[0,[[5,6],0]]"),
            ("[12,0]", "[[6,6],0]"),
            ("[0,12]", "[0,[6,6]]"),
        ] {
            println!("== test_split: {}", input);
            let mut tree: Tree = input.parse()?;
            dbg!(&tree);
            tree.split();
            let want = want.parse()?;
            assert_eq!(tree, want, "\nInput {} Got {} Want {}", input, tree, want);
        }
        Ok(())
    }

    #[test]
    fn test_magnitude() -> Result<()> {
        for (input, want) in [
            ("[9,1]", 29),
            ("[1,9]", 21),
            ("[[9,1],[1,9]]", 129),
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ] {
            let tree: Tree = input.parse()?;
            assert_eq!(tree.magnitude(), want);
        }
        Ok(())
    }

    #[test]
    fn test_add_and_reduce() -> Result<()> {
        for (input, want) in [
            (
                r#"
[1,1]
[2,2]
[3,3]
[4,4]
"#,
                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            ),
            (
                r#"
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
    "#
                .trim(),
                "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            ),
            (
                r#"
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
    "#
                .trim(),
                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            ),
            (
                r#"
[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]
"#
                .trim(),
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            ),
        ] {
            println!("== 1. test_add_and_reduce: {}", input);
            let mut num = sum(input.trim());
            println!("before reduce: {}", num);
            num.reduce();
            println!("after reduce: {}", num);
            assert_eq!(num.to_string(), want);
        }

        for (l, r, eq) in [
            (
                "[[[[4,3],4],4],[7,[[8,4],9]]]",
                "[1,1]",
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            ),
            (
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            ),
            (
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            ),
            (
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            ),
            (
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            ),
            (
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            ),
            (
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
                "[2,9]",
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            ),
            (
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            ),
            (
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            ),
            (
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
                "[[[[4,2],2],6],[8,7]]",
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ] {
            let l: Tree = l.parse()?;
            let r: Tree = r.parse()?;
            let mut num = l + r;
            println!("== 2. test_add_and_reduce: {}", num);
            println!("before reduce: {}", num);
            num.reduce();
            println!("after reduce: {}", num);
            assert_eq!(num.to_string(), eq);
        }
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = r#"
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
    "#
        .trim();

        assert_eq!(part1(input)?, 3488);

        let input = r#"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "#
        .trim();
        assert_eq!(part1(input)?, 4140);
        Ok(())
    }

    /*
    #[test]
    fn test_part2()->Result<()> {
    let input = r#"
    "#
    .trim();
    assert_eq!(part2(input)?, usize::MAX);
    Ok(())
    }
    */
}
