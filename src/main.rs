use std::{fs::File, io::{BufRead, BufReader}};


fn read_input(file:  &str) -> (Vec<usize>, Vec<usize>) {
     let file = File::open(file).expect("file not found!");
     let reader = BufReader::new(file);
     let lines = reader.lines().into_iter();

     let mut ret1 = vec![];
     let mut ret2 = vec![];
     for line in lines {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split("   ").collect();
            ret1.push(parts[0].parse::<usize>().unwrap());
            ret2.push(parts[1].parse::<usize>().unwrap());
        }
     }

     (ret1, ret2)
}

fn sum_diffs(l1: &Vec<usize>, l2: &Vec<usize>) -> usize {
    let mut l1 = l1.clone();
    let mut l2 = l2.clone();
    l1.sort();
    l2.sort();
    let mut sum = 0;
    for i in 0..l1.len() {
         sum += l1[i].abs_diff(l2[i]);
    }
    sum
}


fn main() {
    println!("Hello, world!");
    let (l1, l2) = read_input("src/day1.txt");
    println!("sum_diffs: {}", sum_diffs(&l1, &l2));
}
