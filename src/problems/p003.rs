extern crate thrigger_primes;

pub fn solve(verb: bool) {
    if verb {
        println!("This is function 3");
    }

    let mut input: i64 = 600851475143;
    let mut result = vec![];
    let mut i = 3;

    while input>i {
        if input % i == 0 {
            result.push(i);
            input = input/i;
        }
        // TODO i = thrigger_primes::get_next_prime(i);
        i+=2
    }

    if verb {
        println!("Other primes: {:?}",result);
    }
    println!("{}", input);
}
