fn parse_reports(text: &str) -> Vec<Vec<u32>> {
    text.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input/day02.txt").unwrap();
    let reports = parse_reports(&input);

    let part1_result = part1(&reports);
    println!("part 1 result: {part1_result}");
}

fn valid_report(report: &[u32]) -> bool {
    for i in 0..report.len() - 1 {
        let a = report[i];
        let b = report[i + 1];
        if a == b {
            return false;
        }
        if (a > b) != (report[0] > report[1]) {
            return false;
        }

        let diff = a.abs_diff(b);
        if diff > 3 {
            return false;
        }
    }
    return true;
}

fn part1(reports: &[Vec<u32>]) -> usize {
    reports.iter().filter(|r| valid_report(*&r)).count()
}
