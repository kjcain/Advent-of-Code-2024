use std::path::PathBuf;

mod day_1;

fn main() {
    // print the pwd
    let pwd = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", pwd);

    let input = PathBuf::from("day_1/input.txt");
    let output = PathBuf::from("day_1/output_part_1.txt");

    let (left, right) = day_1::load_locations(input);

    let result = day_1::part_1(left, right);

    println!("Result: {}", result);

    // if output is present, check if the result is correct
    if output.exists() {
        let expected_result: i64 = std::fs::read_to_string(output).unwrap().parse().unwrap();
        assert_eq!(result, expected_result);
    } else {
        std::fs::write(output, result.to_string()).unwrap();
    }
}