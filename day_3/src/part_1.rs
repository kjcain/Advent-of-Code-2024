use regex::Regex;
use std::path::PathBuf;

fn load_program(path: PathBuf) -> String {
    std::fs::read_to_string(path)
        .unwrap()
        .trim_end()
        .to_string()
}

fn part_1(program: String) -> i64 {
    let mul_regex = Regex::new(r"mul\((?<l>\d+),(?<r>\d+)\)").unwrap();

    mul_regex
        .captures_iter(&program)
        .map(|cap| {
            let l: i64 = cap["l"].parse().unwrap();
            let r: i64 = cap["r"].parse().unwrap();
            l * r
        })
        .sum()
}

fn main() {
    let input = PathBuf::from("day_3/input.txt");
    let output = PathBuf::from("day_3/output_part_1.txt");

    let program = load_program(input);

    let result = part_1(program);

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
    fn day_3_part_1_example() {
        let input = PathBuf::from("input_example_part_1.txt");
        let output = PathBuf::from("output_example_part_1.txt");

        let expected_result: i64 = std::fs::read_to_string(output)
            .unwrap()
            .trim_end()
            .to_string()
            .parse()
            .unwrap();

        let program = load_program(input);

        let result = part_1(program);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn day_3_part_1() {
        let input = PathBuf::from("input.txt");
        let output = PathBuf::from("output_part_1.txt");

        let expected_result: i64 = std::fs::read_to_string(output)
            .unwrap()
            .trim_end()
            .to_string()
            .parse()
            .unwrap();

        let program = load_program(input);

        let result = part_1(program);

        assert_eq!(result, expected_result);
    }
}
