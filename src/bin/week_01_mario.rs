use cs50_rust::get_int;

fn main() {
    let size = get_int("Please insert how many rows you want: ");
    println!("{}", create_pyramid(size));
}

fn create_pyramid(size: u32) -> String {
    let mut line = 1;
    let mut pyramid = String::new();

    while line <= size {
        let empty_spaces = " ".repeat((size - line) as usize);
        let hash_tags = "#".repeat(line as usize);
        let new_line = format!("{empty_spaces}{hash_tags}  {hash_tags}\n");
        pyramid.push_str(&new_line);
        line += 1;
    }
    pyramid
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_piramid_with_1_returns_single_line() {
        let result = create_pyramid(1);
        assert_eq!(result, "#  #\n");
    }
    #[test]
    fn create_piramid_with_2_returns_with_break_and_spaces() {
        let result = create_pyramid(4);
        assert_eq!(
            result,
            "   #  #
  ##  ##
 ###  ###
####  ####\n"
        );
    }
}
