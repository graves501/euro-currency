use std::io;

fn main() {
    println!("Insert a number to be converted into Euro currency!");

    let mut number_input = String::new();
    io::stdin()
        .read_line(&mut number_input)
        .expect("Insert floating number!");

    let number_to_convert: f64 = number_input.trim().parse().unwrap_or(0.0);

    if number_to_convert == 0.0 {
        println!("Please insert a (floating) number!");
    } else {
        let currency_as_string = convert_into_euro(number_to_convert);
        println!("{}", currency_as_string);
    }
}

fn convert_into_euro(mut number_to_convert: f64) -> String {
    let mut banknote_500_counter = 0;
    let mut banknote_200_counter = 0;
    let mut banknote_100_counter = 0;
    let mut banknote_50_counter = 0;
    let mut banknote_20_counter = 0;
    let mut banknote_10_counter = 0;
    let mut banknote_5_counter = 0;
    let mut coin_2_counter = 0;
    let mut coin_1_counter = 0;
    let mut coin_50_cent_counter = 0;
    let mut coin_20_cent_counter = 0;
    let mut coin_10_cent_counter = 0;
    let mut coin_5_cent_counter = 0;
    let mut coin_2_cent_counter = 0;
    let mut coin_1_cent_counter = 0;

    const FIVE_HUNDRED: f64 = 500_f64;
    const TWO_HUNDRED: f64 = 200_f64;
    const ONE_HUNDRED: f64 = 100_f64;
    const FIFTY: f64 = 50_f64;
    const TWENTY: f64 = 20_f64;
    const TEN: f64 = 10_f64;
    const FIVE: f64 = 5_f64;
    const TWO: f64 = 2_f64;
    const ONE: f64 = 1_f64;
    const FIFTY_CENT: f64 = 0.5_f64;
    const TWENTY_CENT: f64 = 0.2_f64;
    const TEN_CENT: f64 = 0.1_f64;
    const FIVE_CENT: f64 = 0.05_f64;
    const TWO_CENT: f64 = 0.02_f64;
    const ONE_CENT: f64 = 0.01_f64;

    let mut conversion_result = String::new();
    conversion_result += &format!("\nEuro conversion for number: {}\n\n", number_to_convert);

    while number_to_convert > 0.0 {
        if number_to_convert >= FIVE_HUNDRED {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, FIVE_HUNDRED);
            banknote_500_counter += 1;
        } else if number_to_convert >= TWO_HUNDRED {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, TWO_HUNDRED);
            banknote_200_counter += 1;
        } else if number_to_convert >= ONE_HUNDRED {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, ONE_HUNDRED);
            banknote_100_counter += 1;
        } else if number_to_convert >= FIFTY {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, FIFTY);
            banknote_50_counter += 1;
        } else if number_to_convert >= TWENTY {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, TWENTY);
            banknote_20_counter += 1;
        } else if number_to_convert >= TEN {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, TEN);
            banknote_10_counter += 1;
        } else if number_to_convert >= FIVE {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, FIVE);
            banknote_5_counter += 1;
        } else if number_to_convert >= TWO {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, TWO);
            coin_2_counter += 1;
        } else if number_to_convert >= ONE {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, ONE);
            coin_1_counter += 1;
        } else if number_to_convert >= FIFTY_CENT {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, FIFTY_CENT);
            coin_50_cent_counter += 1;
        } else if number_to_convert >= TWENTY_CENT {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, TWENTY_CENT);
            coin_20_cent_counter += 1;
        } else if number_to_convert >= TEN_CENT {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, TEN_CENT);
            coin_10_cent_counter += 1;
        } else if number_to_convert >= FIVE_CENT {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, FIVE_CENT);
            coin_5_cent_counter += 1;
        } else if number_to_convert >= TWO_CENT {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, TWO_CENT);
            coin_2_cent_counter += 1;
        } else if number_to_convert >= ONE_CENT {
            number_to_convert = subtract_and_round_to_two_decimal_places(number_to_convert, ONE_CENT);
            coin_1_cent_counter += 1;
        }
    }

    if banknote_500_counter > 0 {
        conversion_result += &format!("500 Euro notes: {}\n", banknote_500_counter);
    }

    if banknote_200_counter > 0 {
        conversion_result += &format!("200 Euro notes: {}\n", banknote_200_counter);
    }

    if banknote_100_counter > 0 {
        conversion_result += &format!("100 Euro notes: {}\n", banknote_100_counter);
    }

    if banknote_50_counter > 0 {
        conversion_result += &format!("50 Euro notes: {}\n", banknote_50_counter);
    }

    if banknote_20_counter > 0 {
        conversion_result += &format!("20 Euro notes: {}\n", banknote_20_counter);
    }

    if banknote_10_counter > 0 {
        conversion_result += &format!("10 Euro notes: {}\n", banknote_10_counter);
    }

    if banknote_5_counter > 0 {
        conversion_result += &format!("5 Euro notes: {}\n", banknote_5_counter);
    }

    if coin_2_counter > 0 {
        conversion_result += &format!("2 Euro coins: {}\n", coin_2_counter);
    }

    if coin_1_counter > 0 {
        conversion_result += &format!("1 Euro coins: {}\n", coin_1_counter);
    }

    if coin_50_cent_counter > 0 {
        conversion_result += &format!("50 cent coins: {}\n", coin_50_cent_counter);
    }

    if coin_20_cent_counter > 0 {
        conversion_result += &format!("20 cent coins: {}\n", coin_20_cent_counter);
    }

    if coin_10_cent_counter > 0 {
        conversion_result += &format!("10 cent coins: {}\n", coin_10_cent_counter);
    }

    if coin_5_cent_counter > 0 {
        conversion_result += &format!("5 cent coins: {}\n", coin_5_cent_counter);
    }

    if coin_2_cent_counter > 0 {
        conversion_result += &format!("2 cent coins: {}\n", coin_2_cent_counter);
    }

    if coin_1_cent_counter > 0 {
        conversion_result += &format!("1 cent coins: {}\n", coin_1_cent_counter);
    }

    conversion_result
}

fn subtract_and_round_to_two_decimal_places(mut number_to_subtract: f64, subtraction_value: f64) -> f64 {
    number_to_subtract -= subtraction_value;
    round_to_two_decimal_places(number_to_subtract)
}

fn round_to_two_decimal_places(number_to_round: f64) -> f64 {
    (number_to_round * 100.0).round() / 100.0
}
