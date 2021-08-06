extern crate thrigger_primes;

pub fn solve(verb: bool) {
    if verb {
        println!("This is problem 3");
    }

    let mut input: i64 = 600851475143;
    let mut result = vec![];
    let mut i = 3;

    while input>i {
        if input % i == 0 {
            result.push(i);
            input = input/i;
        }
        i = thrigger_primes::get_next(i);
    }

    if verb {
        println!("Other primes: {:?}",result);
    }
    println!("{}", input);
}
