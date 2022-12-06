use std::{iter::FromIterator, collections::VecDeque};

#[derive(Debug)]
struct Crane {
    pub stacks: Vec<VecDeque<Option<char>>>
}

impl Crane {
    fn new() -> Self {
        Self {
            stacks: vec![VecDeque::new(); 9],
        }
    }

    fn push(&mut self, i: usize, c: Option<char>) {
        if c.is_some() {
            self.stacks[i].push_front(c);
        }
    }

    fn top(&mut self) -> String {
        self.stacks.clone().into_iter()
            .flat_map(|mut x| x.pop_front().expect("should be a char"))
            .collect::<String>()
    }
}

impl<'a> FromIterator<&'a str> for Crane {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut crane = Crane::new();

        for val in iter.into_iter() {
            let xs = val
                .chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .into_iter()
                .map(|xs| {
                    if xs[1].is_whitespace() {
                        None
                    } else {
                        Some(xs[1])
                    }
                })
                .collect::<Vec<_>>();

            let itr = xs.into_iter().collect::<Vec<Option<_>>>();

            for i in 0..itr.len() {
                crane.push(i, itr[i]);
            }
        }

        crane
    }
}

#[derive(Debug)]
struct Moves {
    q: Vec<(usize, usize, usize)>
}

impl<'a> FromIterator<&'a str> for Moves {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut moves: Vec<(usize, usize, usize)> = vec![];

        for i in iter.into_iter() {
            let xs = i.split_whitespace()
                .into_iter()
                .flat_map(|s| s.parse::<usize>())
                .collect::<Vec<_>>();

            moves.push((xs[0], xs[1], xs[2]));
        }

        Moves { q: moves }
    }
}

fn main() {
    let (a, b) = include_str!("./input.txt")
        .split_once("\n\n")
        .expect("aoc");

    let mut a = a.lines().rev().skip(1).collect::<Crane>();
    let b = b.lines().collect::<Moves>();

    for (mv, from, to) in b.q {
        let f = from - 1;
        let t = to - 1;
        for _ in 0..mv {
            let e = a.stacks[f].pop_front().expect("should be a value here");
            a.stacks[t].push_front(e);
        }
    }

    println!("{:?}", a.top());
}
