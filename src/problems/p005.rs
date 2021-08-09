pub fn solve() -> u64 {
    let max_test_value = 20;

    let mut current_value = max_test_value;

    loop {
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

    current_value
}

