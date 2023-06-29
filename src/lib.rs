use std::io;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn get_int(message: &str) -> u32 {
    println!("{message}");
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input.");

        match input.trim().parse::<u32>() {
            Ok(input) => return input,
            Err(_) => {
                println!("Invalid input. Please insert a number!");
                continue;
            }
        };
    }
}

pub fn get_float(message: &str) -> f32 {
    println!("{message}");
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input.");

        match input.trim().parse::<f32>() {
            Ok(input) => return input,
            Err(_) => {
                println!("Invalid input. Please insert a number!");
                continue;
            }
        };
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
