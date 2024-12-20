use std::path::PathBuf;

fn load_reports(path: PathBuf) -> Vec<Vec<i64>> {
    let input = std::fs::read_to_string(path)
        .unwrap()
        .trim_end()
        .to_string();

    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_2_safety(report: Vec<i64>) -> bool {
    if report.len() < 2 {
        return false;
    }

    let increasing = report[0] < report[1];

    for i in 1..report.len() {
        let curr = report[i - 1];
        let next = report[i];

        let locally_increasing = next > curr;
        let abs_diff = next.abs_diff(curr);

        if !(1..=3).contains(&abs_diff) {
            return false;
        }

        if locally_increasing != increasing {
            return false;
        }
    }

    true
}

fn part_2_safety_shotgun(report: Vec<i64>) -> bool {
    let safe_as_is = part_2_safety(report.clone());

    if safe_as_is {
        return true;
    }

    if !safe_as_is {
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);

            if part_2_safety(report) {
                return true;
            }
        }
    }

    false
}

fn part_2(reports: Vec<Vec<i64>>) -> i64 {
    reports
        .iter()
        .filter(|&report| part_2_safety_shotgun(report.to_vec()))
        .count() as i64
}

fn main() {
    let input = PathBuf::from("day_2/input.txt");
    let output = PathBuf::from("day_2/output_part_2.txt");

    let reports = load_reports(input);

    let result = part_2(reports);

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
    fn day_2_part_2_example() {
        let input = PathBuf::from("input_example.txt");
        let output = PathBuf::from("output_example_part_2.txt");

        let expected_result: i64 = std::fs::read_to_string(output)
            .unwrap()
            .trim_end()
            .to_string()
            .parse()
            .unwrap();

        let reports = load_reports(input);

        let result = part_2(reports);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn day_2_part_2() {
        let input = PathBuf::from("input.txt");
        let output = PathBuf::from("output_part_2.txt");

        let expected_result: i64 = std::fs::read_to_string(output)
            .unwrap()
            .trim_end()
            .to_string()
            .parse()
            .unwrap();

        let reports = load_reports(input);

        let result = part_2(reports);

        assert_eq!(result, expected_result);
    }
}
