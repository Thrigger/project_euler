pub fn solve() -> u64 {
    let max_range = 100;

    let mut sum_of_square = 0;
    for i in 1..max_range + 1 {
        sum_of_square += i*i;
    }

    let mut square_of_sum = 0;
    for i in 1..max_range + 1 {
        square_of_sum += i;
    }
    square_of_sum *= square_of_sum;

    let result = square_of_sum - sum_of_square;

    result
}

