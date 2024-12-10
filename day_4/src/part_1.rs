use std::path::PathBuf;

static DAY: &str = "day_4";
static PART: &str = "part_1";

fn load_problem(path: PathBuf) -> String {
    std::fs::read_to_string(path).expect("Error reading file")
}

#[derive(Debug)]
struct Match {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

impl PartialEq for Match {
    fn eq(&self, other: &Self) -> bool {
        self.start_x == other.start_x
            && self.start_y == other.start_y
            && self.end_x == other.end_x
            && self.end_y == other.end_y
    }
}

fn search_2d_part(
    problem: &Vec<Vec<char>>,
    search_str: &str,
    start_x: i32,
    start_y: i32,
    step_x: i32,
    step_y: i32,
) -> Option<Match> {
    let x_bound = step_x * (search_str.len() as i32);
    let y_bound = step_y * (search_str.len() as i32);

    let x_max = problem[0].len() as i32;
    let y_max = problem.len() as i32;

    let mut extracted_match = String::new();

    for i in 0..search_str.len() as i32 {
        let x = start_x + (i * step_x);
        let y = start_y + (i * step_y);

        if x < 0 || x >= x_max {
            return None;
        }

        if y < 0 || y >= y_max {
            return None;
        }

        extracted_match.push(problem[y as usize][x as usize]);
    }

    if extracted_match == search_str {
        let m = Match {
            start_x: start_x as usize,
            start_y: start_y as usize,
            end_x: (start_x + x_bound) as usize,
            end_y: (start_y + y_bound) as usize,
        };

        return Some(m);
    }

    None
}

fn search_2d(
    problem: &Vec<Vec<char>>,
    search_str: &str,
    start_x: usize,
    start_y: usize,
) -> Vec<Match> {
    let mut matches: Vec<Match> = Vec::new();

    let search_steps = vec![
        (1, 0),   // right
        (1, -1),  // down-right
        (0, -1),  // down
        (-1, -1), // down-left
        (-1, 0),  // left
        (-1, 1),  // up-left
        (0, 1),   // up
        (1, 1),   // up-right
    ];

    for search_step in search_steps.iter() {
        match search_2d_part(
            problem,
            search_str,
            start_x as i32,
            start_y as i32,
            search_step.0,
            search_step.1,
        ) {
            Some(m) => {
                matches.push(m);
            }
            None => {}
        }
    }
            

    matches
}

fn locate_matches(problem: &str, search_str: &str) -> Vec<Match> {
    let problem_matrix = problem
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut matches = Vec::new();

    for row in 0..problem_matrix.len() {
        for col in 0..problem_matrix[row].len() {
            let new_matches: Vec<Match> = search_2d(&problem_matrix, search_str, col, row);

            matches.extend(new_matches);
        }
    }
    
    matches
}

fn solve(problem: String) -> i64 {
    let matches = locate_matches(&problem, "XMAS");

    matches.len() as i64
}

fn test_solution(input_path: PathBuf, output_path: PathBuf, write_output: bool) {
    let input = load_problem(input_path);
    let result = solve(input);

    println!("Result: {}", result);

    if output_path.exists() {
        let expected_result: i64 = std::fs::read_to_string(output_path)
            .unwrap()
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
