pub fn solve(verb: bool) {
    let max_range = 100;

    if verb {
        println!("This is problem 5");
    }

    let mut sum_of_square = 0;
    for i in 1..max_range + 1 {
        sum_of_square += i*i;
    }

    let mut square_of_sum = 0;
    for i in 1..max_range + 1 {
        square_of_sum += i;
    }
    square_of_sum *= square_of_sum;

    println!("{}", square_of_sum - sum_of_square);
}

