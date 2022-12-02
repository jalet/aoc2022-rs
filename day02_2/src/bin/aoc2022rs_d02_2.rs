// A = X => Rock     => 1p
// B = Y => Paper    => 2p
// C = Z => Scissors => 3p
//
// Scissors > Paper
// Rock     > Scissors
// Paper    > Rock
fn main() {
    let res = include_str!("./input.txt")
        .lines()
        .map(|l| match l.chars().collect::<Vec<char>>().as_slice() {
            [a, _, 'X'] => fnl(a),
            [a, _, 'Y'] => fnd(a),
            [a, _, 'Z'] => fnw(a),
            _ => 0,
        })
        .sum::<u32>();

    println!("{:?}", res)
}

fn fnl(c: &char) -> u32 {
    match c {
        'A' => 0 + 3,
        'B' => 0 + 1,
        'C' => 0 + 2,
        _ => 0,
    }
}

fn fnd(c: &char) -> u32 {
    match c {
        'A' => 3 + 1,
        'B' => 3 + 2,
        'C' => 3 + 3,
        _ => 0,
    }
}

fn fnw(c: &char) -> u32 {
    match c {
        'A' => 6 + 2,
        'B' => 6 + 3,
        'C' => 6 + 1,
        _ => 0,
    }
}
