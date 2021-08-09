mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;
mod p007;

mod p010;

pub fn solve(problem: i32, verbose: bool) -> u64 {
    let solution = match problem {
        1 => p001::solve(),
        2 => p002::solve(),
        3 => p003::solve(),
        4 => p004::solve(),
        5 => p005::solve(),
        6 => p006::solve(),
        7 => p007::solve(),
        10 => p010::solve(),
        _ => 0,
    };

    solution
}
