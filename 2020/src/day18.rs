//! --- Day 18: Operation Order ---
//! As you look out the window and notice a heavily-forested continent slowly appear over the horizon, you are interrupted by the child sitting next to you. They're curious if you could help them with their math homework.
//!
//! Unfortunately, it seems like this "math" follows different rules than you remember.
//!
//! The homework (your puzzle input) consists of a series of expressions that consist of addition (+), multiplication (*), and parentheses ((...)). Just like normal math, parentheses indicate that the expression inside must be evaluated before it can be used by the surrounding expression. Addition still finds the sum of the numbers on both sides of the operator, and multiplication still finds the product.
//!
//! However, the rules of operator precedence have changed. Rather than evaluating multiplication before addition, the operators have the same precedence, and are evaluated left-to-right regardless of the order in which they appear.
//!
//! For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are as follows:
//!
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!       9   + 4 * 5 + 6
//!          13   * 5 + 6
//!              65   + 6
//!                  71
//! Parentheses can override this order; for example, here is what happens if parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):
//!
//! 1 + (2 * 3) + (4 * (5 + 6))
//! 1 +    6    + (4 * (5 + 6))
//!      7      + (4 * (5 + 6))
//!      7      + (4 *   11   )
//!      7      +     44
//!             51
//! Here are a few more examples:
//!
//! 2 * 3 + (4 * 5) becomes 26.
//! 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
//! 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
//! ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
//! Before you can help with the homework, you need to understand it yourself. Evaluate the expression on each line of the homework; what is the sum of the resulting values?

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Num(u32),
    Add,
    Mul,
    Open,
    Close,
    Space,
}

#[derive(Debug, PartialEq)]
enum Ast {
    Add(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Num(u32),
}

fn eval(ast: &Ast) -> u32 {
    0
}

#[derive(Copy, Clone, Default, Debug)]
struct NodeId(usize);

#[derive(Debug, Default)]
struct Node {
    parent: NodeId,
    data: Option<Token>,
    left: Option<NodeId>,
    right: Option<NodeId>,
}

impl Node {
    fn set_left_child(&mut self, id: Option<NodeId>) -> Option<NodeId> {
        let old = self.left;
        self.left = id;
        old
    }
    fn set_right_child(&mut self, id: Option<NodeId>) -> Option<NodeId> {
        let old = self.right;
        self.right = id;
        old
    }
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
}

use std::fmt;
impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cur = NodeId(0);

        fn print_node(f: &mut fmt::Formatter<'_>, tree: &Tree, id: NodeId) -> fmt::Result {
            match tree.get_node(id).data {
                None => (),
                Some(Token::Num(n)) => write!(f, "{}", n)?,
                Some(Token::Add) => {
                    print_node(f, tree, tree.get_node(id).left.unwrap())?;
                    write!(f, "+",)?;
                    print_node(f, tree, tree.get_node(id).right.unwrap())?;
                }
                Some(Token::Mul) => {
                    print_node(f, tree, tree.get_node(id).left.unwrap())?;
                    write!(f, "*",)?;
                    print_node(f, tree, tree.get_node(id).right.unwrap())?;
                }
                Some(Token::Open) => {
                    write!(f, "(",)?;
                }
                Some(Token::Close) => {
                    write!(f, ")",)?;
                }
                Some(Token::Space) => {
                    write!(f, " ",)?;
                }
            };
            Ok(())
        }
        print_node(f, self, cur)
    }
}
impl Default for Tree {
    fn default() -> Tree {
        Tree {
            nodes: vec![Node::default()],
        }
    }
}

impl Tree {
    fn get_node(&self, id: NodeId) -> &Node {
        &self.nodes[id.0]
    }
    fn get_node_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.0]
    }
    fn add_node(&mut self, n: Node) -> NodeId {
        self.nodes.push(n);
        NodeId(self.nodes.len() - 1)
    }
}

#[derive(Default)]
struct Parser {
    tree: Tree,
    cur: NodeId,
    debug_tokens: Vec<Token>,
}

impl Parser {
    fn add_left_child_cur(&mut self, mut n: Node) -> NodeId {
        n.parent = self.cur;
        let id = self.tree.add_node(n);
        self.cur_node_mut().left = Some(id);
        id
    }
    fn add_right_child_cur(&mut self, mut n: Node) -> NodeId {
        n.parent = self.cur;
        let id = self.tree.add_node(n);
        self.cur_node_mut().right = Some(id);
        id
    }
    fn cur_node_mut(&mut self) -> &mut Node {
        self.tree.get_node_mut(self.cur)
    }

    fn parse(&mut self, tokens: &[Token]) {
        self.debug_tokens = tokens.to_vec();
        tokens.iter().for_each(|t| {
            self.add_token(t);
        });
    }
    fn add_token(&mut self, t: &Token) {
        match t {
            Token::Num(n) => {
                self.cur_node_mut().data = Some(*t);
                self.cur = self.cur_node_mut().parent;
            }
            Token::Add => {
                self.cur_node_mut().data = Some(*t);
                let id = self.add_right_child_cur(Node::default());
                self.cur = id;
            }
            Token::Mul => {
                self.cur_node_mut().data = Some(*t);
                let id = self.add_right_child_cur(Node::default());
                self.cur = id;
            }
            Token::Open => {
                let id = self.add_left_child_cur(Node::default());
                self.cur = id;
            }
            Token::Close => self.cur = self.cur_node_mut().parent,
            Token::Space => unreachable!("spaces should be filtered"),
        };
    }
    fn eval(&self) -> u32 {
        dbg!(&self.debug_tokens);
        dbg!(&self.tree);
        fn eval_inner(tree: &Tree, id: NodeId) -> u32 {
            match tree.get_node(id).data {
                None => panic!("empty node"),
                Some(Token::Num(n)) => n,
                Some(Token::Add) => {
                    eval_inner(tree, tree.get_node(id).left.unwrap())
                        + eval_inner(tree, tree.get_node(id).right.unwrap())
                }
                Some(Token::Mul) => {
                    eval_inner(tree, tree.get_node(id).left.unwrap())
                        * eval_inner(tree, tree.get_node(id).right.unwrap())
                }
                Some(Token::Open) => panic!("("),
                Some(Token::Close) => panic!(")"),
                Some(Token::Space) => panic!(" "),
            }
        }
        eval_inner(&self.tree, NodeId(0))
    }
}

#[aoc_generator(day18)]
fn lex(input: &str) -> Vec<Token> {
    input
        .bytes()
        .map(|b| match b {
            b'(' => Token::Open,
            b')' => Token::Close,
            b'+' => Token::Add,
            b'*' => Token::Mul,
            b' ' => Token::Space,
            c @ b'0'..=b'9' => Token::Num((c - b'0') as u32),
            c => panic!(format!("unexpected char '{:?}'", c)),
        })
        // Ignore spaces
        .filter(|t| t != &Token::Space)
        .collect()
}

fn parse(tokens: &[Token]) -> u32 {
    let mut p = Parser::default();
    p.parse(tokens);
    p.eval()
}

#[aoc(day18, part1)]
fn solution1(tokens: &[Token]) -> u32 {
    parse(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let tokens = vec![
            Token::Num(2),
            Token::Mul,
            Token::Num(3),
            Token::Add,
            Token::Open,
            Token::Num(4),
            Token::Mul,
            Token::Num(5),
            Token::Close,
        ];
        assert_eq!(lex("2 * 3 + (4 * 5)"), tokens);
    }

    #[test]
    fn part1() {
        for (input, want) in vec![
            ("(1 + 2 * 3 + 4 * 5 + 6)", 71),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ] {
            assert_eq!(solution1(&lex(input)), want, "{}", input);
        }
    }
}
