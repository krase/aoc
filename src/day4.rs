
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

    if row + 2 < height && col + 2 < width {
        let mut tmp1 = vec![];
        tmp1.push(data[row][col]);
        tmp1.push(data[row+1][col+1]);
        tmp1.push(data[row+2][col+2]);
        
        let row = row+ 2;
        let mut tmp2 = vec![];
        tmp2.push(data[row][col]);
        tmp2.push(data[row-1][col+1]);
        tmp2.push(data[row-2][col+2]);
        if test_vec(&mut count, &tmp1) &&
        test_vec(&mut count, &tmp2) {
            count += 1;
        }
    }

    count
}

fn test_vec(count: &mut usize, xx: &[u8]) -> bool {
    if b"MAS".eq(xx) || b"SAM".eq(xx) {
        return true;
    }
    
    false
}

fn test_all(data: &Matrix) -> usize{
    let mut counter: usize = 0;
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if data[row][col] == b'M' || data[row][col] == b'S'{
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