extern crate support;

pub fn solve() -> u64 {
    let a_max = 5;
    let b_max = 5;

    let mut results = vec![];

    for a in 0..a_max {
        for b in 0..b_max {
            let product = thrigger_support::pow(a, b);

            let mut unique = true;
            for each in results {
                if each == product {
                    unique = false;
                    break;
                }
            }
            if unique {
                results.push(product);
            }
        }
    }
    result.len()
}

