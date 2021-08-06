pub fn solve(verb: bool) {
    if verb {
        println!("This is problem 1");
    }

    let mut sum = 0;
    for i in 1..1000 {
        if i % 5 == 0 || i % 3 == 0 {
            sum += i;
        }
    }

    println!("{}",sum);
}
