
use std::{fs::File, io::{BufReader, Read}};

fn read_input(file:  &str) -> String {
    let file = File::open(file).expect("file not found!");
    let mut reader = BufReader::new(file);
    let mut buf: Vec<u8> = vec![];
    let _ = reader.read_to_end(&mut buf);
    String::from_utf8(buf.to_vec()).unwrap()
}

enum States {
    FindMul = 1,
    FindOpen,
    FindDigits1,
    FindComma,
    FindDigits2,
    FindClose,
}

struct State {
    num1: usize,
    num2: usize,
    state: States,
}

fn reset(state: &mut State) {
    state.num1 = 0;
    state.num2 = 0;
    state.state = States::FindMul;
}

fn scan(content: &str) -> usize {
    let mut state  = State {
        num1: 0,
        num2: 0,
        state: States::FindMul,
    };
    let mut enabled = true;
    let mut pos = 0;
    let mut accu = 0usize;
    while pos < content.len() {
        match state.state {
            States::FindMul => {
                let tmp = &content[pos..];
                if tmp.starts_with("do()") {
                    enabled = true;
                    pos += 4;
                }
                if tmp.starts_with("don't()") {
                    enabled = false;
                    pos += 7;
                }

                if content[pos..].starts_with("mul") {
                    state.state = States::FindOpen;
                    pos += 2;
                } else {
                    reset(&mut state);
                }
                pos += 1;
            }
            States::FindOpen => {
                if content[pos..].starts_with("(") {
                    state.state = States::FindDigits1;
                } else {
                    reset(&mut state);
                }
                pos += 1;
            }
            States::FindClose => {
                if content[pos..].starts_with(")") {
                    state.state = States::FindMul;
                    if enabled {
                        accu += state.num1 * state.num2;
                    }
                    println!("{} * {}", state.num1, state.num2);
                } else {
                    reset(&mut state);
                }
                pos += 1;                
            }
            States::FindComma => {
                let tmp = &content[pos..];
                if tmp.as_bytes()[0] == ',' as u8 {
                    state.state = States::FindDigits2;
                } else {
                    reset(&mut state);
                }
                pos += 1;
            }
            States::FindDigits1 => {
                let mut end = 0;
                let mut amount = 0;
                for (i, c) in content[pos..].as_bytes().iter().enumerate() {
                    if c.is_ascii_digit() {
                        end = i+1;
                        amount += 1;
                    } else {
                        break;
                    }
                }
                if amount > 0 && amount < 4 {
                    let tmp = &content[pos..pos+end];
                    let number: usize = tmp.parse().unwrap();
                    state.num1 = number;
                    state.state = States::FindComma;
                } else {
                    reset(&mut state);
                }
                pos += amount;
            }
            States::FindDigits2 => {
                let mut end = 0;
                let mut amount = 0;
                for (i, c) in content[pos..].as_bytes().iter().enumerate() {
                    if c.is_ascii_digit() {
                        end = i+1;
                        amount += 1;
                    } else {
                        break;
                    }
                }
                if amount > 0 && amount < 4 {
                    let tmp = &content[pos..pos+end];
                    let number: usize = tmp.parse().unwrap();
                    state.num2 = number;
                    state.state = States::FindClose;
                } else {
                    reset(&mut state);
                }
                pos += amount;
            }
        }
    }
    accu
}


fn main() {
    println!("Hello, world!");
    //let data = read_input("src/example3.txt");
    let data = read_input("src/day3.txt");
    
    println!("mul sum: {}", scan(&data));
}