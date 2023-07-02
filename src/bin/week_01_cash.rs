use cs50_rust::get_int;

fn main() {
    let change = get_int("Please insert the total change: ");
    let (quarters, change_left) = calculate_change(change, Coin::Quarter);
    let (dimes, change_left) = calculate_change(change_left, Coin::Dime);
    let (nickels, change_left) = calculate_change(change_left, Coin::Nickel);
    let (pennies, _) = calculate_change(change_left, Coin::Penny);

    println!("Quarters ($.25): {quarters}\nDimes ($.10): {dimes}\nNickels ($.05): {nickels}\nPennies ($.01): {pennies}\n");
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value(&self) -> u32 {
        match *self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn calculate_change(change: u32, coin: Coin) -> (u32, u32) {
    let mut total_coins = 0;
    let mut change_left = change;

    while change_left >= coin.value() {
        total_coins += 1;
        change_left -= coin.value();
    }
    (total_coins, change_left)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculate_change_with_100_returns_4_quarters() {
        let result = calculate_change(100, Coin::Quarter);
        assert_eq!(result, (4, 0));
    }
    #[test]
    fn calculate_change_with_99_returns_3_quarters_and_change_left_matches() {
        let result = calculate_change(99, Coin::Quarter);
        assert_eq!(result, (3, 24));
    }
}
