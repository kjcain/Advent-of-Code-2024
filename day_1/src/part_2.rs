use std::path::PathBuf;

pub fn load_locations(path: PathBuf) -> (Vec<i64>, Vec<i64>) {
    let input = std::fs::read_to_string(path).unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }

    (left, right)
}

pub fn part_2(left: Vec<i64>, right: Vec<i64>) -> i64 {
    // count how many times each number appears in the right list
    let mut right_count = std::collections::HashMap::new();

    for item in right {
        *right_count.entry(item).or_insert(0) += 1;
    }

    let mut result = 0;

    for i in 0..left.len() {
        if !right_count.contains_key(&left[i]) {
            continue;
        }

        result += left[i] * right_count[&left[i]];
    }

    result
}

fn main() {
    // print the pwd
    let pwd = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", pwd);

    let input = PathBuf::from("day_1/input.txt");
    let output = PathBuf::from("day_1/output_part_2.txt");

    let (left, right) = load_locations(input);

    let result = part_2(left, right);

    println!("Result: {}", result);

    // if output is present, check if the result is correct
    if output.exists() {
        let expected_result: i64 = std::fs::read_to_string(output).unwrap().parse().unwrap();
        assert_eq!(result, expected_result);
    } else {
        std::fs::write(output, result.to_string()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn day_1_part_2_example() {
        let input = PathBuf::from("input_example.txt");
        let output = PathBuf::from("output_example_part_2.txt");

        let expected_result: i64 = std::fs::read_to_string(output).unwrap().parse().unwrap();

        let (left, right) = load_locations(input);

        let result = part_2(left, right);
        
        assert_eq!(result, expected_result);
    }

    #[test]
    fn day_1_part_2() {
        let input = PathBuf::from("input.txt");
        let output = PathBuf::from("output_part_2.txt");

        let expected_result: i64 = std::fs::read_to_string(output).unwrap().parse().unwrap();

        let (left, right) = load_locations(input);

        let result = part_2(left, right);
        
        assert_eq!(result, expected_result);
    }
}
