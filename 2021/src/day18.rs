use advent::prelude::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::io::Read;

#[derive(Debug, PartialEq)]
enum ChildType {
    None,
    Value(usize),
    Subtree(Idx),
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Idx(usize);

#[derive(Debug, PartialEq)]
struct Node {
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
#[derive(Debug, Default, PartialEq)]
struct Tree {
    root: Idx,
    nodes: Vec<Node>,
}

fn read_byte<R: Read>(reader: &mut R) -> std::io::Result<Option<u8>> {
    reader.bytes().next().transpose()
}

impl Tree {
    fn merge(left: &Tree, right: &Tree) -> Tree {
        // This is lazy but works for simple any obvious reasons (if FromStr and Display work
        // correctly).
        format!("[{},{}]", left, right)
            .parse()
            .expect("failed to parse merge tree")
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
            idx,
            parent: None,
            left,
            right,
        });
        idx
    }

    fn from_str_node<R: Read>(&mut self, r: &mut R) -> ChildType {
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
                    let v = dbg!(b - b'0');
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
            ChildType::Value(v) => write!(f, "{}", v),
            ChildType::Subtree(idx) => self.fmt_node(f, &self[idx]),
        };
        write!(f, ",")?;
        match node.right {
            ChildType::None => panic!("right node was None"),
            ChildType::Value(v) => write!(f, "{}", v),
            ChildType::Subtree(idx) => self.fmt_node(f, &self[idx]),
        };
        write!(f, "]")?;
        Ok(())
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
        tree.from_str_node(&mut bytes);
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

#[aoc(day18, part1)]
fn part1(input: &str) -> Result<usize> {
    for l in input.lines() {
        let tree: Tree = l.parse()?;
        dbg!(&tree);
    }
    Ok(0)
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
    fn test_merge() -> Result<()> {
        assert_eq!(
            Tree::merge(&"[1,2]".parse().unwrap(), &"[[3,4],5]".parse().unwrap()),
            "[[1,2],[[3,4],5]]".parse().unwrap()
        );
        Ok(())
    }

    //#[test]
    fn test_part1() -> Result<()> {
        let sum = [
            [[[6, 6], [7, 6]], [[7, 7], [7, 0]]],
            [[[7, 7], [7, 7]], [[7, 8], [9, 9]]],
        ];
        dbg!(&sum);
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
