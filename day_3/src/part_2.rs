use regex::Regex;
use std::path::PathBuf;

fn load_program(path: PathBuf) -> String {
    std::fs::read_to_string(path)
        .unwrap()
        .trim_end()
        .to_string()
        .replace("\n", "")
}

fn part_2(program: String) -> i64 {
    let mul_regex = Regex::new(r"mul\((?<l>\d+),(?<r>\d+)\)").unwrap();
    let inactive_code_regex = Regex::new(r"don't\(\).*?do\(\)").unwrap();

    let active_program = inactive_code_regex.replace_all(&program, "").to_string();

    mul_regex
        .captures_iter(&active_program)
        .map(|cap| {
            let l: i64 = cap["l"].parse().unwrap();
            let r: i64 = cap["r"].parse().unwrap();

            l * r
        })
        .sum()
}

fn main() {
    let input = PathBuf::from("day_3/input.txt");
    let output = PathBuf::from("day_3/output_part_2.txt");

    let program = load_program(input);

    let result = part_2(program);

    println!("Result: {}", result);

    // if output is present, check if the result is correct
    if output.exists() {
        let expected_result: i64 = std::fs::read_to_string(output)
            .unwrap()
            .trim_end()
            .to_string()
            .parse()
            .unwrap();
        assert_eq!(result, expected_result);
    } else {
        std::fs::write(output, result.to_string()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_3_part_2_example() {
        let input = PathBuf::from("input_example_part_2.txt");
        let output = PathBuf::from("output_example_part_2.txt");

        let expected_result: i64 = std::fs::read_to_string(output)
            .unwrap()
            .trim_end()
            .to_string()
            .parse()
            .unwrap();

        let program = load_program(input);

        let result = part_2(program);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn day_3_part_2() {
        let input = PathBuf::from("input.txt");
        let output = PathBuf::from("output_part_2.txt");

        let expected_result: i64 = std::fs::read_to_string(output)
            .unwrap()
            .trim_end()
            .to_string()
            .parse()
            .unwrap();

        let program = load_program(input);

        let result = part_2(program);

        assert_eq!(result, expected_result);
    }
}
