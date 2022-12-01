fn main() {
    let res = include_str!("./input.txt")
        .trim()
        .split("\n\n")
        .map(|x| {
            return x.split("\n")
             .map(|x| x.parse::<usize>().unwrap())
        })
        .map(|x| x.sum::<usize>())
        .max();

    print!("{:?}", res);
}
