use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn move_crate(&mut self, count: usize, from: usize, to: usize) {
        for _ in 0..count {
            let v = self.stacks[from - 1].pop().expect("popped empty stack");
            self.stacks[to - 1].push(v);
        }
    }
    fn tops(&self) -> String {
        self.stacks.iter().map(|s| s[s.len() - 1]).collect()
    }
}

fn build_stacks(top: &str) -> Stacks {
    let lines: Vec<_> = top.split('\n').collect();

    let mut stacks: Vec<Vec<_>> = vec![Vec::new(); (lines[0].len() + 1) / 4];

    lines.iter().map(|l| l.as_bytes()).for_each(|l| {
        let mut i = 1;
        let mut c = 0;
        while i < l.len() {
            if l[i] != b' ' {
                stacks[c].push(l[i].into());
            }
            i += 4;
            c += 1;
        }
    });
    // Flip stacks upside down.
    let stacks = stacks
        .into_iter()
        .map(|mut s| {
            // Skip number at bottom of text.
            s.truncate(s.len() - 1);
            s.reverse();
            s
        })
        .collect();
    Stacks { stacks }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let (top, bottom) = input.split_once("\n\n").unwrap();
    let mut s = build_stacks(top);
    println!("{:#?}", s);
    for line in bottom.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        s.move_crate(
            parts[1].parse().expect("couldn't parse number"),
            parts[3].parse().expect("couldn't parse number"),
            parts[5].parse().expect("couldn't parse number"),
        );
    }
    s.tops()
}

// #[aoc(day5, part2)]
// fn part2(input: &str) -> usize { }
