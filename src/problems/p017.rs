pub fn solve() -> u64 {

    let mut total_letters = 0;
    let mut i = 1;

    while i< 1001{
        total_letters += letters_in(i);
        i += 1;
    }

    total_letters
}

fn letters_in(start_number: u64) -> u64 {
    let mut value = start_number;
    let mut total_letters = 0;

    while value > 0 {
        if value >= 1000 {
            total_letters += 11;
            value = value % 1000;
        } else if value >= 100 {
            if value % 100 != 0 {
                total_letters += 3;
            }
            total_letters += letters_below_10(value / 100) + 7;
            value = value % 100;
        } else if value >= 10 {
            total_letters += letters_below_100(value);
            value = 0;
        } else {
            total_letters += letters_below_10(value);
            value = value / 10;
        }
    }

    total_letters
}

fn letters_below_100(number: u64) -> u64 {
    if number < 20 {
        match number {
            10 => 3,
            11 | 12 => 6,
            15 | 16 => 7,
            13 | 14 | 18 | 19 => 8,
            17 => 9,
            _ => panic!(),
        }
    } else {
        let deci = match number / 10 {
            4 | 5 | 6 => 5,
            2 | 3 | 8 | 9 => 6,
            7 => 7,
            _ => panic!(),
        };
        deci + letters_below_10(number%10)
    }
}

fn letters_below_10(number: u64) -> u64 {
    match number {
        1| 2| 6 => 3,
        4| 5| 9 => 4,
        3| 7| 8 => 5,
        0 => 0,
        _ => panic!(),
    }
}

