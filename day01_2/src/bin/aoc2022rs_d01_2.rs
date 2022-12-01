fn main() {
    let mut res = include_str!("./input.txt")
        .trim()
        .split("\n\n")
        .map(|x| {
            return x.split("\n")
             .map(|x| x.parse::<usize>().unwrap())
        })
        .map(|x| x.sum::<usize>())
        .collect::<Vec<_>>();

    res.sort_by(|a, b| b.cmp(a));

    print!("{:?}", res.iter().take(3).sum::<usize>());
}

