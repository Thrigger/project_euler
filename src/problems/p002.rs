pub fn solve(verb: bool) {
    if verb {
        println!("This is function 2");
    }

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
    println!("{}",sum);
}
