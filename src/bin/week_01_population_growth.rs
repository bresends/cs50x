use cs50_rust::get_int;

fn main() {
    let mut initial_population = get_int("Please insert initial population (9+): ");

    while initial_population < 9 {
        initial_population = get_int("Please insert a number greater than 9: ")
    }

    let mut final_population = get_int("Please insert the final population: ");

    while final_population < initial_population {
        final_population = get_int("Please insert a number greater than the initial population: ")
    }

    println!(
        "Years: {}",
        years_to_reach_population(initial_population, final_population)
    )
}

fn years_to_reach_population(initial_population: u32, final_population: u32) -> u32 {
    let mut current_population: u32 = initial_population;
    let mut years_passed: u32 = 0;

    while current_population < final_population {
        years_passed += 1;
        current_population += current_population / 3 - current_population / 4;
    }
    years_passed
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn years_to_reach_population_should_return_0_when_equals() {
        let result = years_to_reach_population(10, 10);
        assert_eq!(result, 0);
    }

    #[test]
    fn years_to_reach_population_default_values() {
        let result = years_to_reach_population(1200, 1300);
        assert_eq!(result, 1);
    }
    #[test]
    fn years_to_reach_population_ten_false() {
        let result = years_to_reach_population(100, 1_000_000u32);
        assert_eq!(result, 115);
    }
}
