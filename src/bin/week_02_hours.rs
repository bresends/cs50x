use cs50_rust::get_int;
use std::io;

fn main() {
    let number_of_weeks = get_int("Number of weeks taking CS50: ");
    let mut hours: Vec<u32> = Vec::new();

    for i in 0..number_of_weeks {
        print!("Week {i} HW Hours: ");
        let hours_studied = get_int("");
        hours.push(hours_studied);
    }

    let operation = get_operation();

    if operation == "T" {
        println!("Total hours studied: {}", calc_total(hours));
    } else {
        println!("Average hours studied: {}", calc_average(&hours));
    }
}

fn get_operation() -> String {
    println!("Enter T for total hours, A for average hours per week: ");
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read the input.");

        match input.to_uppercase().trim() {
            "A" | "T" => return input.trim().to_uppercase(),
            _ => {
                println!("Please insert T for total or A for average.");
                continue;
            }
        }
    }
}

fn calc_total(week_hours: Vec<u32>) -> u32 {
    week_hours.iter().sum()
}

fn calc_average(numbers: &[u32]) -> f32 {
    numbers.iter().sum::<u32>() as f32 / numbers.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculate_total_returns_desired_value() {
        let result = calc_total(vec![1, 2, 3]);
        assert_eq!(result, 6);
    }
    #[test]
    fn calculate_average_returns_desired_value() {
        let result = calc_average(&vec![1, 2, 3]);
        assert_eq!(result, 2.0);
    }
}
