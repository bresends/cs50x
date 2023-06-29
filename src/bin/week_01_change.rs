use cs50_rust::get_float;

fn main() {
    let bill = get_float("Please insert the bill: ");
    let tax = get_float("Please insert the tax (%): ");
    let tip = get_float("Please insert the tip (%): ");

    let split_bill = split_bill(bill, tax, tip);

    println!("\nBill before tax and tip: {bill}");
    println!("Sale Tax Percent: {tax}");
    println!("Tip Percent: {tax}");
    println!("You will owe {split_bill:.2} each!");
}

fn split_bill(bill: f32, tax: f32, tip: f32) -> f32 {
    let bill_after_tax = bill + bill * (tax / 100.0);
    let bill_after_tip = bill_after_tax + bill_after_tax * (tip / 100.0);
    bill_after_tip / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_values_match() {
        let result = split_bill(100.0, 6.25, 18.0);
        assert_eq!(result, 62.6875);
    }
    #[test]
    fn default_values_match() {
        let result = split_bill(12.50, 8.875, 20.0);
        assert_eq!(result, 8.165625);
    }
}
