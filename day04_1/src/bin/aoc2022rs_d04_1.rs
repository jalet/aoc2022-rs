use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T>
where
    T: PartialOrd + Debug,
{
    fn new(a: T, b: T) -> Self {
        Self { a, b }
    }

    fn contains(&self, o: &Pair<T>) -> bool {
        self.a <= o.a && self.b >= o.b || o.a <= self.a && o.b >= self.b
    }
}

impl<T> FromStr for Pair<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("-").expect("Pair::FromStr expect '-'");
        let a = a.parse::<T>().expect("Couldn't parse lhs of Pair");
        let b = b.parse::<T>().expect("Couldn't parse rhs of Pair");
        Ok(Pair { a, b })
    }
}

fn main() {
    let res = include_str!("./input.txt")
        .lines()
        .map(|l| {
            l.split_once(",")
                .map(|(a, b)| {
                    Pair::new(
                        a.parse::<Pair<usize>>().expect(""),
                        b.parse::<Pair<usize>>().expect(""),
                    )
                })
                .expect("")
        })
        .filter(|x| x.a.contains(&x.b))
        .count();

    println!("{:?}", res);
}
