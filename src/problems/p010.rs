extern crate thrigger_primes;

pub fn solve(verb: bool) {
    if verb {
        println!("This is problem 10");
    }

    let max_prime = 2000000;
    let mut current_prime = 2;

    let primes = thrigger_primes::get_primes(max_prime);

    let mut prime_sum = 0;
    for prime in primes {
        prime_sum += prime;
    }

    println!("{}", prime_sum);
}

