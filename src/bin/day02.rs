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
    let mut reports = parse_reports(&input);

    let part1_result = part1(&reports);
    println!("part 1 result: {part1_result}");

    let part2_result = part2(&mut reports);
    println!("part 1 result: {part2_result}");
}

fn valid_report(report: &[u32]) -> Result<(), usize> {
    for i in 0..report.len() - 1 {
        let a = report[i];
        let b = report[i + 1];
        if a == b {
            return Err(i);
        }
        if (a > b) != (report[0] > report[1]) {
            return Err(i);
        }

        let diff = a.abs_diff(b);
        if diff > 3 {
            return Err(i);
        }
    }
    Ok(())
}

fn valid_report_with_tolerance(r: &mut Vec<u32>) -> bool {
    let Err(i) = valid_report(r) else {
        // if there is no error, the report is valid without removing a single level.
        return true;
    };

    let original_report = r.clone();
    // We can remove the number at the index or the one after to try to fix the report.
    let x = r[i];
    r.remove(i);
    if valid_report(r).is_ok() {
        return true;
    }
    // Removing the number at the reported index failed; we can try to remove the number following the reported error location instead.
    // We do this by replacing the number at the location (the one originally following the reported location) with the number we originally removed.
    r[i] = x;
    if valid_report(r).is_ok() {
        return true;
    }

    // There is one more way to potentially fix the report: by removing the first element of the original report.
    // This can change whether elements have to be smaller or larger than the subsequent element.
    let report = &original_report[1..];
    valid_report(report).is_ok()
}

fn part1(reports: &[Vec<u32>]) -> usize {
    reports
        .iter()
        .map(|r| valid_report(r))
        .filter(|r| r.is_ok())
        .count()
}

fn part2(reports: &mut [Vec<u32>]) -> usize {
    reports
        .iter_mut()
        .map(valid_report_with_tolerance)
        .filter(|b| *b)
        .count()
}
