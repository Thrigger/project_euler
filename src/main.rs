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
                Ok(i) => println!("{}", problems::solve(i, verbose)),
                _=>(),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(problems::solve(1, false), 233168);
        assert_eq!(problems::solve(2, false), 4613732);
        assert_eq!(problems::solve(3, false), 6857);
        assert_eq!(problems::solve(4, false), 906609);
        assert_eq!(problems::solve(5, false), 232792560);
        assert_eq!(problems::solve(6, false), 25164150);
        assert_eq!(problems::solve(7, false), 104743);
        assert_eq!(problems::solve(10, false), 142913828922);
    }
}

