
use std::fs::File;
use std::io::{BufRead, BufReader};

type Matrix = Vec<Vec<u8>>;

fn read_input(file:  &str) -> Matrix {
    let file = File::open(file).expect("file not found!");
    let reader = BufReader::new(file);
    let lines = reader.lines().into_iter();

    let mut ret = vec![];
    for line in lines {
        if let Ok(line) = line {
            ret.push(line.as_bytes().to_vec());
        }
    }

    ret
}

fn test_xmas(data: &Matrix, row: usize, col: usize) -> usize {
    let height = data.len();
    let width = data[0].len();
    let mut count = 0usize;
    if col + 3 < width {
        let tmp = &data[row][col..col+4];
        test_vec(&mut count, tmp);
        
    }
    //up down
    if row + 3 < height {
        let tmp = &data[row..row+4];
        let mut xx = vec![];
        for r in tmp {
            xx.push(r[col]);
        }
        test_vec(&mut count, &xx);
        if col + 3 < width {
            let mut tmp = vec![];
            tmp.push(data[row][col]);
            tmp.push(data[row+1][col+1]);
            tmp.push(data[row+2][col+2]);
            tmp.push(data[row+3][col+3]);
            test_vec(&mut count, &tmp);
        }
        if col >= 3 {
            let mut tmp = vec![];
            tmp.push(data[row][col]);
            tmp.push(data[row+1][col-1]);
            tmp.push(data[row+2][col-2]);
            tmp.push(data[row+3][col-3]);
            test_vec(&mut count, &tmp);
        }
    }

    count
}

fn test_vec(count: &mut usize, xx: &[u8]) {
    if b"XMAS".eq(xx) {
        *count += 1;
    }
    if b"SAMX".eq(xx) {
        *count += 1;
    }
}

fn test_all(data: &Matrix) -> usize{
    let mut counter: usize = 0;
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if data[row][col] == b'X' || data[row][col] == b'S'{
                counter += test_xmas(data, row, col);
            }
        }
    }
    counter
}


fn main() {
    println!("Hello, world!");
    //let data = read_input("src/example4.txt");
    let data = read_input("src/day4.txt");

    println!("count: {}", test_all(&data));
}