mod p001;
mod p002;
mod p003;
mod p004;

pub fn solve(problem: i32, verbose: bool) {
    match problem {
        1 => p001::solve(verbose),
        2 => p002::solve(verbose),
        3 => p003::solve(verbose),
        4 => p004::solve(verbose),
        _ => (),
    };
}
