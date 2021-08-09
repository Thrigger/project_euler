extern crate thrigger_primes;

pub fn solve() -> u64 {
    let max_prime = 2000000;
    let primes = thrigger_primes::get_primes(max_prime);
    let mut prime_sum = 0;

    for prime in primes {
        prime_sum += prime;
    }

    prime_sum
}

