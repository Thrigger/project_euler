extern crate thrigger_primes;

pub fn solve(verb: bool) {
    let number_of_primes = 10001;
    /* I know 2 is a prime so start at 2 */
    let mut found_primes = 1;

    if verb {
        println!("This is problem 7");
    }

    let mut current_prime = 2;

    while found_primes < number_of_primes {
        current_prime = thrigger_primes::get_next(current_prime);
        
        found_primes += 1;
    }

    println!("{}", current_prime);
}

