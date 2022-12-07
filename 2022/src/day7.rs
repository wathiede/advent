use std::fmt;

use aoc_runner_derive::aoc;

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: usize,
}

#[derive(Debug)]
struct Dirent<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
    dirs: Vec<usize>,
    // Total of all children.
    size: usize,
    // My place in dir list.
    idx: usize,
    // My parent's place in dir list.
    parent_idx: usize,
}

#[derive(Debug)]
struct Filesystem<'a> {
    dirs: Vec<Dirent<'a>>,
    cwd: usize,
}
impl<'a> Filesystem<'a> {
    fn new() -> Filesystem<'a> {
        Filesystem {
            dirs: vec![Dirent {
                name: "/",
                files: Vec::new(),
                dirs: Vec::new(),
                size: 0,
                idx: 0,
                parent_idx: 0,
            }],
            cwd: 0,
        }
    }
    fn dirs_at_most(&self, size: usize) -> usize {
        self.dirs
            .iter()
            .filter(|d| d.size <= size)
            .map(|d| d.size)
            .sum()
    }
    fn mkdir(&mut self, path: &'a str) -> usize {
        let next_idx = self.dirs.len();
        let cur = &self.dirs[self.cwd];
        self.dirs.push(Dirent {
            name: path,
            files: Vec::new(),
            dirs: Vec::new(),
            size: 0,
            idx: next_idx,
            parent_idx: cur.idx,
        });
        let cur = &mut self.dirs[self.cwd];
        cur.dirs.push(next_idx);
        next_idx
    }
    fn up(&mut self) {
        let cur = &self.dirs[self.cwd];
        self.cwd = cur.parent_idx;
    }
    fn cd(&mut self, path: &'a str) {
        let cur = &self.dirs[self.cwd];

        let o_idx = cur
            .dirs
            .iter()
            .map(|idx| &self.dirs[*idx])
            .position(|d| d.name == path);
        dbg!(o_idx, path);
        let idx = if let Some(child_idx) = o_idx {
            cur.dirs[child_idx]
        } else {
            self.mkdir(path)
        };
        self.cwd = idx;
    }
    fn touch(&mut self, path: &'a str, size: usize) {
        let cur = &mut self.dirs[self.cwd];
        if cur.files.iter().position(|d| d.name == path).is_none() {
            cur.files.push(File { name: path, size });
            // Walk up my parents adding size.
            let mut idx = cur.idx;
            while idx != 0 {
                let d = &mut self.dirs[idx];
                d.size += size;
                idx = d.parent_idx;
            }
            self.dirs[0].size += size;
        }
    }

    fn fmt_dirent(&self, f: &mut fmt::Formatter<'_>, idx: usize, indent: usize) -> fmt::Result {
        for d in &self.dirs[idx].dirs {
            writeln!(f, "{} - {} (dir)", " ".repeat(indent), self.dirs[*d].name)?;
            self.fmt_dirent(f, *d, indent + 1)?
        }
        for file in &self.dirs[idx].files {
            writeln!(
                f,
                "{} - {} (file, size={})",
                " ".repeat(indent),
                file.name,
                file.size,
            )?;
        }
        Ok(())
    }
}

// - / (dir)
//  - a (dir)
//    - e (dir)
//      - i (file, size=584)
//    - f (file, size=29116)
//    - g (file, size=2557)
//    - h.lst (file, size=62596)
//  - b.txt (file, size=14848514)
//  - c.dat (file, size=8504156)
//  - d (dir)
//    - j (file, size=4060174)
//    - d.log (file, size=8033020)
//    - d.ext (file, size=5626152)
//    - k (file, size=7214296)
impl<'a> fmt::Display for Filesystem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "- / (dir)")?;
        self.fmt_dirent(f, 0, 0)
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut fs = Filesystem::new();
    input.lines().for_each(|l| {
        println!("{l}");
        let parts: Vec<_> = l.split(" ").collect();
        match parts.as_slice() {
            ["$", "cd", "/"] => (),
            ["$", "cd", ".."] => fs.up(),
            ["$", "cd", path] => {
                fs.cd(path);
            }
            ["$", "ls"] => (),
            ["dir", name] => {
                fs.mkdir(name);
            }
            [size, name] => fs.touch(name, size.parse().expect("not a number")),
            _ => panic!("unexpected pattern: {}", l),
        };
    });
    dbg!(&fs);
    println!("FS:\n{}", &fs);
    fs.dirs_at_most(100000)
}

#[test]
fn p1() {
    assert_eq!(
        part1(
            r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#
        ),
        95437
    );
}
// #[aoc(day7, part2)]
// fn part2(input: &str) -> usize { }
