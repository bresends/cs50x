use cs50_rust::get_int;

fn main() {
    let lower_range = get_int("Please insert the minimum number: ");
    let upper_range = get_int("Please insert the maximum number: ");

    for number in lower_range..=upper_range {
        if is_prime(number) == true {
            println!("{number}");
        }
    }
}

fn is_prime(number: u32) -> bool {
    if number == 1 {
        return false;
    }

    let limit = (number as f64).sqrt() as u32;

    for division_number in 2..=limit {
        if number % division_number == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_prime_one_true() {
        let result = is_prime(1);
        assert_eq!(result, false);
    }

    #[test]
    fn is_prime_two_true() {
        let result = is_prime(2);
        assert_eq!(result, true);
    }
    #[test]
    fn is_prime_ten_false() {
        let result = is_prime(10);
        assert_eq!(result, false);
    }
}
