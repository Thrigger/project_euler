extern crate thrigger_primes;

pub fn solve() -> u64 {
    let mut input: u64 = 600851475143;
    let mut result = vec![];
    let mut i = 3;

    while input>i {
        if input % i == 0 {
            result.push(i);
            input = input/i;
        }
        i = thrigger_primes::get_next(i);
    }

    input
}
