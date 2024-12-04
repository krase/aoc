
use std::{fs::File, io::{BufRead, BufReader}};


fn read_input(file:  &str) -> Vec<Vec<usize>> {
    let file = File::open(file).expect("file not found!");
    let reader = BufReader::new(file);
    let lines = reader.lines().into_iter();

    let mut ret = vec![];
    for line in lines {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(" ").collect();
            ret.push(parts.iter().map(|x| x.parse::<usize>().unwrap()).collect());
        }
    }

    ret
}

fn test_report(report: &Vec<usize>) -> bool {
    if !(report.iter().is_sorted() || report.iter().rev().is_sorted()) {
        return false;
    }

    for (i, val) in report.iter().enumerate() {
        if i > 0 {
            let diff = val.abs_diff(report[i - 1]);
            if diff > 3 || diff < 1 {
                return false;
            }
        }
    }
    return true;
}

fn safe_reports(reports: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for report in reports {
        if test_report(report) {
            count += 1;
        }
    }
    count
}

fn safe_reports2(reports: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for report in reports {
        if test_report(report) {
            count += 1;
        } else {
            for i in 0..report.len() {
                let mut tmp = report.clone();
                tmp.remove(i);
                if test_report(&tmp) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}


fn main() {
    println!("Hello, world!");
    //let data = read_input("src/example2.txt");
    let data = read_input("src/day2.txt");
    
    println!("report: {}", safe_reports2(&data));
}
