pub fn solve() -> u64 {
    let mut sum = 0;
    let mut current  = 2;
    let mut previous = 1;
    while current < 4000000 {
        if current%2 == 0 {
            sum += current;
        }
        let temp = previous;
        previous = current;
        current += temp;
    }

    sum
}
