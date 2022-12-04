use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug)]
struct Pair<'a> {
    pub fst: &'a str,
    pub snd: &'a str,
}

impl<'a> Pair<'a> {
    fn new(fst: &'a str, snd: &'a str) -> Self {
        Self { fst, snd }
    }
}

impl<'a> Display for Pair<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{fst={}, snd={}}}", self.fst, self.snd)
    }
}

impl<'a> From<&'a str> for Pair<'a> {
    fn from(item: &'a str) -> Self {
        let item = item.split_at(item.len() / 2);

        Pair::new(item.0, item.1)
    }
}

#[derive(Debug)]
struct Priority(u32);

impl From<char> for Priority {
    fn from(item: char) -> Self {
        let p = match item.is_lowercase() {
            true => item as u32 - 96,
            _ => item as u32 - 64 + 26,
        };

        dbg!(item, p);
        Priority(p)
    }
}

impl From<Priority> for u32 {
    fn from(item: Priority) -> Self {
        item.0
    }
}

fn main() {
    let res = include_str!("./input.txt")
        .lines()
        .map(|x| Pair::from(x))
        .flat_map(|p| {
            let mut out = HashSet::new();

            for c in p.fst.chars() {
                if p.snd.contains(c) {
                    out.insert(c);
                }
            }

            out
        })
        .map(|c| Priority::from(c))
        .map(|p| u32::from(p))
        .sum::<u32>();

    println!("{:?}", res);
}
