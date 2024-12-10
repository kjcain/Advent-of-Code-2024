use std::{collections::HashMap, path::PathBuf};

static DAY: &str = "day_5";
static PART: &str = "part_2";

fn load_problem(path: PathBuf) -> String {
    std::fs::read_to_string(path)
        .expect("Error reading file")
        .trim_end()
        .to_string()
}

fn extract_problem(problem: &str) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    // split on blank line
    let mut parts = problem.split("\n\n");

    let mut ordering_rules = HashMap::new();

    for line in parts.next().unwrap().lines() {
        let (left, right) = line.split_once("|").unwrap();

        let left = left.trim().parse::<i64>().unwrap();
        let right = right.trim().parse::<i64>().unwrap();

        ordering_rules.entry(left).or_insert(Vec::new()).push(right);
    }

    let mut updates = Vec::new();

    for line in parts.next().unwrap().lines() {
        let mut update = Vec::new();

        for num in line.split(",") {
            update.push(num.parse::<i64>().unwrap());
        }

        updates.push(update);
    }

    (ordering_rules, updates)
}

fn is_properly_ordered(ordering_rules: &HashMap<i64, Vec<i64>>, update: &[i64]) -> bool {
    // attempting w/o recursion for now

    // left must come before all right
    for i in 0..update.len() {
        let left = update[i];

        for update_item in update.iter().skip(i + 1) {
            if ordering_rules.contains_key(update_item)
                && ordering_rules[update_item].contains(&left)
            {
                return false;
            }
        }
    }

    true
}

fn order_update(ordering_rules: &HashMap<i64, Vec<i64>>, update: &[i64]) -> Vec<i64> {
    let mut ordered_update = Vec::new();

    for &new_page in update {
        if ordered_update.is_empty() {
            ordered_update.push(new_page);
        } else {
            let mut inserted = false;

            for j in 0..ordered_update.len() {
                let page = ordered_update[j];

                if ordering_rules.contains_key(&page) && ordering_rules[&page].contains(&new_page) {
                    ordered_update.insert(j, new_page);
                    inserted = true;
                    break;
                }
            }

            if !inserted {
                ordered_update.push(new_page);
            }
        }
    }

    ordered_update
}

fn solve(problem: String) -> i64 {
    let (ordering_rules, updates) = extract_problem(&problem);

    updates
        .iter()
        .filter(|update| !is_properly_ordered(&ordering_rules, update))
        .map(|update| order_update(&ordering_rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn test_solution(input_path: PathBuf, output_path: PathBuf, write_output: bool) {
    let input = load_problem(input_path);
    let result = solve(input);

    println!("Result: {}", result);

    if output_path.exists() {
        let expected_result: i64 = std::fs::read_to_string(output_path)
            .unwrap()
            .trim_end()
            .to_string()
            .parse()
            .unwrap();

        assert_eq!(result, expected_result);
    } else if write_output {
        std::fs::write(output_path, result.to_string()).unwrap();
    }
}

fn main() {
    let input = PathBuf::from(format!("{}/input.txt", DAY));
    let output = PathBuf::from(format!("{}/output_{}.txt", DAY, PART));

    test_solution(input, output, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = PathBuf::from(format!("input_example_{}.txt", PART));
        let output = PathBuf::from(format!("output_example_{}.txt", PART));

        test_solution(input, output, false);
    }

    #[test]
    fn problem() {
        let input = PathBuf::from("input.txt");
        let output = PathBuf::from(format!("output_{}.txt", PART));

        test_solution(input, output, false);
    }
}
