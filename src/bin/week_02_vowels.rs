use std::{env, process};

fn main() {
    let mut arg: Vec<String> = env::args().collect();
    let words = parse_input(&mut arg).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("{}", replace(words));
}

fn parse_input(args: &mut Vec<String>) -> Result<String, &'static str> {
    if args.len() <= 1 {
        return Err("Please insert a word!");
    }

    args.remove(0);

    let phrase = args.join(" ");
    Ok(phrase)
}
fn replace(phrase: String) -> String {
    let result = phrase
        .replace("a", "6")
        .replace("e", "3")
        .replace("i", "1")
        .replace("o", "0");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn parse_inputs_should_throw_when_no_arguments_sent() {
        let result = parse_input(&mut vec![]).unwrap();
        assert_eq!(result, "");
    }
    #[test]
    fn parse_inputs_returns_desired_value() {
        let result = parse_input(&mut vec![
            "Path".to_string(),
            "Hello".to_string(),
            "World".to_string(),
        ])
        .unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn replace_returns_desired_value() {
        let result = replace("Hello".to_string());
        assert_eq!(result, "H3ll0");
    }
}
