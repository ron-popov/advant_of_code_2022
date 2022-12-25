use log::{info, debug, error};

fn get_snafu_digit_decimal_value(c: &char) -> isize {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Invalid snafu digit")
    }
}

fn get_decimal_digit_snafu_value(i: isize) -> char {
    match i {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!("Invalid snafu value")
    }
}

fn get_base_five_digit(i: isize) -> char {
    match i {
        4 => '4',
        3 => '3',
        2 => '2',
        1 => '1',
        0 => '0',
        _ => panic!("Invalid base5 digit")
    }
}

fn blabla(i: isize) -> isize {
    match i {
        5 => 0,
        4 => -1,
        3 => -2,
        2 => 2,
        1 => 1,
        0 => 0,
        -1 => -1,
        -2 => -2,
        _ => panic!("BLABLA WTF? : {}", i)
    }
}

fn base5_to_snafu(_input: &String) -> String {
    "blabla".to_string()
}

fn int_to_base5(input: isize) -> String {
    let mut snafu = input.clone();
    let mut snafu_string: String = "".to_string();

    debug!("Temp Snafu value is {}", snafu);

    while snafu != 0 {
        let remainder: isize = snafu.rem_euclid(5);
        let remainder_char: char = get_base_five_digit(remainder);
        snafu_string.push(remainder_char);

        snafu /= 5;

        debug!("Remainder is {}, which is the char {}, resulting in Temp Snafu : {}", remainder, remainder_char, snafu);
    }

    debug!("Snafu string for {} is {}", input, snafu_string);
    return snafu_string;
}

fn snafu_to_int(input: &String) -> isize {
    let mut snafu: String = input.clone();
    let mut value: isize = 0;
    let mut char_value: isize = 1;

    while snafu.len() != 0 {
        let c: isize = get_snafu_digit_decimal_value(&snafu.pop().unwrap());
        value += c * char_value;
        char_value *= 5;
    }

    return value;
}

pub fn solve(user_input: Vec<String>) {
    info!("Solving challenge 25");
    info!("I got text with {} lines in it", user_input.len());

    assert!(snafu_to_int(&"1=-".to_string()) == 24);
    assert!(snafu_to_int(&"1=-0-2".to_string()) == 1747);
    assert!(int_to_base5(1747) == "1=-0-2");

    let mut snafu_values_sum: isize = 0;

    for line in user_input {
        let decimal_value = snafu_to_int(&line);
        debug!("SNAFU Value {} is {} in decimal", line, decimal_value);

        snafu_values_sum += decimal_value;
    }

    info!("Snafu values sum is {}", snafu_values_sum);
}