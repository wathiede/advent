//! --- Day 22: Crab Combat ---
//! It only takes a few hours of sailing the ocean on a raft for boredom to sink in. Fortunately, you brought a small deck of space cards! You'd like to play a game of Combat, and there's even an opponent available: a small crab that climbed aboard your raft before you left.
//!
//! Fortunately, it doesn't take long to teach the crab the rules.
//!
//! Before the game starts, split the cards so each player has their own deck (your puzzle input). Then, the game consists of a series of rounds: both players draw their top card, and the player with the higher-valued card wins the round. The winner keeps both cards, placing them on the bottom of their own deck so that the winner's card is above the other card. If this causes a player to have all of the cards, they win, and the game ends.
//!
//! For example, consider the following starting decks:
//!
//! Player 1:
//! 9
//! 2
//! 6
//! 3
//! 1
//!
//! Player 2:
//! 5
//! 8
//! 4
//! 7
//! 10
//! This arrangement means that player 1's deck contains 5 cards, with 9 on top and 1 on the bottom; player 2's deck also contains 5 cards, with 5 on top and 10 on the bottom.
//!
//! The first round begins with both players drawing the top card of their decks: 9 and 5. Player 1 has the higher card, so both cards move to the bottom of player 1's deck such that 9 is above 5. In total, it takes 29 rounds before a player has all of the cards:
//!
//! -- Round 1 --
//! Player 1's deck: 9, 2, 6, 3, 1
//! Player 2's deck: 5, 8, 4, 7, 10
//! Player 1 plays: 9
//! Player 2 plays: 5
//! Player 1 wins the round!
//!
//! -- Round 2 --
//! Player 1's deck: 2, 6, 3, 1, 9, 5
//! Player 2's deck: 8, 4, 7, 10
//! Player 1 plays: 2
//! Player 2 plays: 8
//! Player 2 wins the round!
//!
//! -- Round 3 --
//! Player 1's deck: 6, 3, 1, 9, 5
//! Player 2's deck: 4, 7, 10, 8, 2
//! Player 1 plays: 6
//! Player 2 plays: 4
//! Player 1 wins the round!
//!
//! -- Round 4 --
//! Player 1's deck: 3, 1, 9, 5, 6, 4
//! Player 2's deck: 7, 10, 8, 2
//! Player 1 plays: 3
//! Player 2 plays: 7
//! Player 2 wins the round!
//!
//! -- Round 5 --
//! Player 1's deck: 1, 9, 5, 6, 4
//! Player 2's deck: 10, 8, 2, 7, 3
//! Player 1 plays: 1
//! Player 2 plays: 10
//! Player 2 wins the round!
//!
//! ...several more rounds pass...
//!
//! -- Round 27 --
//! Player 1's deck: 5, 4, 1
//! Player 2's deck: 8, 9, 7, 3, 2, 10, 6
//! Player 1 plays: 5
//! Player 2 plays: 8
//! Player 2 wins the round!
//!
//! -- Round 28 --
//! Player 1's deck: 4, 1
//! Player 2's deck: 9, 7, 3, 2, 10, 6, 8, 5
//! Player 1 plays: 4
//! Player 2 plays: 9
//! Player 2 wins the round!
//!
//! -- Round 29 --
//! Player 1's deck: 1
//! Player 2's deck: 7, 3, 2, 10, 6, 8, 5, 9, 4
//! Player 1 plays: 1
//! Player 2 plays: 7
//! Player 2 wins the round!
//!
//!
//! == Post-game results ==
//! Player 1's deck:
//! Player 2's deck: 3, 2, 10, 6, 8, 5, 9, 4, 7, 1
//! Once the game ends, you can calculate the winning player's score. The bottom card in their deck is worth the value of the card multiplied by 1, the second-from-the-bottom card is worth the value of the card multiplied by 2, and so on. With 10 cards, the top card is worth the value on the card multiplied by 10. In this example, the winning player's score is:
//!
//!    3 * 10
//! +  2 *  9
//! + 10 *  8
//! +  6 *  7
//! +  8 *  6
//! +  5 *  5
//! +  9 *  4
//! +  4 *  3
//! +  7 *  2
//! +  1 *  1
//! = 306
//! So, once the game ends, the winning player's score is 306.
//!
//! Play the small crab in a game of Combat using the two decks you just dealt. What is the winning player's score?
use std::collections::VecDeque;

use aoc_runner_derive::aoc;

#[derive(Debug, PartialEq)]
struct Players {
    p1: VecDeque<usize>,
    p2: VecDeque<usize>,
}

fn generator(input: &str) -> Players {
    let players: Vec<_> = input.split("\n\n").collect();
    Players {
        p1: players[0]
            .split('\n')
            .skip(1)
            .map(|s| s.parse().expect("couldn't parse p1 number"))
            .collect::<VecDeque<usize>>(),
        p2: players[1]
            .split('\n')
            .skip(1)
            .map(|s| s.parse().expect("couldn't parse p2 number"))
            .collect::<VecDeque<usize>>(),
    }
}

impl Players {
    fn play(&mut self) {
        //let mut round = 0;
        while !self.p1.is_empty() && !self.p2.is_empty() {
            let p1 = self.p1.pop_front().unwrap();
            let p2 = self.p2.pop_front().unwrap();
            //round += 1;
            //println!("-- Round {} --", round);
            //println!("Player 1's deck: {:?}", self.p1);
            //println!("Player 2's deck: {:?}", self.p2);
            //println!("Player 1 plays: {}", p1);
            //println!("Player 2 plays: {}", p2);
            if p1 > p2 {
                //println!("Play 1 wins the round!");
                self.p1.push_back(p1);
                self.p1.push_back(p2);
            } else {
                //println!("Play 2 wins the round!");
                self.p2.push_back(p2);
                self.p2.push_back(p1);
            }
            //println!();
        }
    }
    fn winning_score(&self) -> usize {
        let winner = if self.p1.len() > self.p2.len() {
            &self.p1
        } else {
            &self.p2
        };
        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, n)| (i + 1) * *n)
            .sum()
    }
}

#[aoc(day22, part1)]
fn solution1(input: &str) -> usize {
    let mut players = generator(input);
    players.play();
    players.winning_score()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;

    #[test]
    fn test_generator() {
        assert_eq!(
            generator(INPUT),
            Players {
                p1: vec![9, 2, 6, 3, 1].into(),
                p2: vec![5, 8, 4, 7, 10].into(),
            }
        );
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(INPUT), 306);
    }
}
