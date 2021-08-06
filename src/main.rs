use std::env;

mod problems;

fn main() {
    let mut args = env::args();
    let mut verbose = false;
    args.next();

    for arg in args {
        if arg == "-v" {
            verbose = true;
        } else {
            match arg.parse::<i32>() {
                Ok(i) => problems::solve(i, verbose),
                _=>(),
            };
        }
    }
}
