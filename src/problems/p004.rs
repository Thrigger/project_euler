pub fn solve() -> u64 {
    let max_value = 999;

    let mut current_value =  max_value * max_value;

    while current_value > 480 {
        /* creating char vector instead of integer */
        let value_as_string = format!("{}", current_value);
        let mut value_as_chars: Vec<char> = value_as_string.chars().collect();

        let mut is_palindrom = true;
        /* loop while the value has more than one number. if it is only one number left then it is
         * a palindrom */
        while value_as_chars.len() > 1 {
            let first = value_as_chars[0];
            value_as_chars.remove(0);
            let last = value_as_chars[value_as_chars.len() - 1];
            value_as_chars.pop();

            if first != last {
                current_value -= 1;
                is_palindrom = false;
                break;
            }
        }

        if is_palindrom {
            if is_multiple(current_value, max_value) {
                break;
            } else {
                current_value -= 1;
            }
        }
        
    }

    current_value
}

fn is_multiple(val: u64, max_val: u64) -> bool {
    let mut i = max_val;
    
    while i > 0 {
        if val % i == 0 && val / i <= max_val {
            return true;
        } else {
            i -= 1;
        }
    }
    false
}

