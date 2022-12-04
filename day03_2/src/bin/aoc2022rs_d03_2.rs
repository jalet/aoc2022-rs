use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rucksack {
    pub val: Vec<char>,
}

impl Rucksack {
    fn new(str: &str) -> Self {
        let val = str
            .chars()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        Self { val }
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
        .map(|x| Rucksack::new(x))
        .collect::<Vec<Rucksack>>();

    let iter = res
        .chunks(3)
        .map(|xs| {
            let mut map: HashMap<char, u32> = HashMap::new();

            for x in xs {
                for c in x.val.clone() {
                    map.insert(
                        c,
                        match map.get(&c) {
                            Some(n) => n + 1,
                            _ => 1,
                        },
                    );
                }
            }

            let mut out: HashSet<char> = HashSet::new();
            for (k, v) in map.iter_mut() {
                if *v as usize >= xs.len() {
                    out.insert(*k);
                }
            }

            out
        })
        .flat_map(|x| x.into_iter().collect::<Vec<_>>())
        .map(|c| u32::from(Priority::from(c)))
        .sum::<u32>();

    println!("{:?}", iter);
}
