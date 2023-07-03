use std::io;

fn main() {
    let credit_card = get_credit_card();
    let card_valid = check_valid(&credit_card);

    if card_valid {
        println!("{}", check_issuer(&credit_card))
    } else {
        println!("Invalid Card")
    }
}

fn check_valid(card_number: &str) -> bool {
    let first_numbers_sum = sum_every_one_digits(&card_number);
    let second_numbers_sum = sum_every_two_digits(&card_number);
    let total_sum = first_numbers_sum + second_numbers_sum;

    if total_sum % 10 == 0 {
        return true;
    }
    false
}

fn check_issuer(card_number: &str) -> String {
    let first_2_digits = &card_number[..2].trim().parse::<u32>().unwrap();

    match first_2_digits {
        34 | 37 => return "American Express".to_string(),
        51..=55 => return "Mastercard".to_string(),
        40..=49 => return "Visa".to_string(),
        _ => return "Invalid Card".to_string(),
    }
}
fn sum_every_two_digits(raw_card_number: &str) -> u32 {
    let multiplied_digits: Vec<_> = raw_card_number
        .chars()
        .rev()
        .skip(1)
        .step_by(2)
        .map(|char| char.to_digit(10).unwrap() * 2)
        .collect();

    let mut sum = 0;

    for number in multiplied_digits {
        if number >= 10 {
            let split_number_sum: u32 = number
                .to_string()
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .sum();
            sum += split_number_sum
        } else {
            sum += number;
        }
    }
    sum
}

fn sum_every_one_digits(raw_card_number: &str) -> u32 {
    raw_card_number
        .chars()
        .rev()
        .step_by(2)
        .map(|char| char.to_digit(10).unwrap())
        .sum::<u32>()
}

fn get_credit_card() -> String {
    println!("Please insert the credit card number: ");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input.");

        let credit_card_lenght = input.trim().len();

        if credit_card_lenght < 13 || credit_card_lenght > 16 {
            println!("Insert a number between 13 and 16 characters.");
            continue;
        }

        match input.trim().parse::<u64>() {
            Ok(input) => return input.to_string(),
            Err(_) => {
                println!("Invalid input. Please insert a credit card number!");
                continue;
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_credit_card_multiplied_chars_matches() {
        let result = sum_every_two_digits("4003600000000014");
        assert_eq!(result, 13);
    }
    #[test]
    fn sum_of_credit_card_non_multiplied_char_matches() {
        let result = sum_every_one_digits("4003600000000014");
        assert_eq!(result, 7);
    }
    #[test]
    fn credit_card_issuer_matches() {
        let result = check_issuer("4003600000000014");
        assert_eq!(result, "Visa");
    }
}
