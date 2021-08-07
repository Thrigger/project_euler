pub fn solve(verb: bool) {
    let max_test_value = 20;

    if verb {
        println!("This is problem 5");
    }

    let mut current_value = max_test_value;

    let solution = loop {
        let mut is_solution = true;
        for i in (1..max_test_value).rev() {
            if current_value % i != 0 {
                is_solution = false;
                break;
            }
        }

        if is_solution {
            break;
        }
    
        current_value += max_test_value;
    };

    println!("{}", current_value);
}

