// A = X => Rock     => 1p
// B = Y => Paper    => 2p
//
// C = Z => Scissors => 3p
// Scissors > Paper
// Rock     > Scissors
// Paper    > Rock
fn main() {
    let res = include_str!("./input.txt")
        .lines()
        .map(|l| match l.chars().collect::<Vec<char>>().as_slice() {
            ['A', _, 'X'] => 4,
            ['B', _, 'Y'] => 5,
            ['C', _, 'Z'] => 6,
            ['C', _, 'X'] => 7,
            ['A', _, 'Y'] => 8,
            ['B', _, 'Z'] => 9,
            [_, _, c] => match c {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => 0,
            },
            _ => 0,
        })
        .sum::<u32>();

    println!("{:?}", res)
}
