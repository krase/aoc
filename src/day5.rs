use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Graph;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Protocol {
    orderings: DiGraph<usize, ()>,
    updates: Vec<Vec<usize>>,
    val2index: HashMap<usize, NodeIndex>,
}

fn read_input(file: &str) -> Protocol {
    let file = File::open(file).expect("file not found!");
    let reader = BufReader::new(file);
    let lines = reader.lines().into_iter();

    let mut ret = Protocol {
        orderings: Graph::new(),
        updates: vec![],
        val2index: HashMap::new(),
    };

    let mut mode = true;
    let mut edges = Vec::<(usize, usize)>::new();
    let mut nodes = HashSet::<usize>::new();

    for (i, line) in lines.enumerate() {
        if let Ok(line) = line {
            if line == "" {
                mode = false;
                continue;
            }
            if mode {
                let (a, b) = line
                    .split_once("|")
                    .ok_or(format!("Invalid line: {}", line))
                    .and_then(|(a, b)| {
                        Ok((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                    })
                    .unwrap();
                println!("{}: ({} {})", i + 1, a, b);
                edges.push((a, b));
                nodes.insert(a);
                nodes.insert(b);
            } else {
                let mut update_line = Vec::new();
                for num in line.split(',').map(|n| n.parse::<usize>().unwrap()) {
                    update_line.push(num);
                }
                ret.updates.push(update_line);
            }
        }
    }

    //let mut val2index: HashMap<usize, NodeIndex> = HashMap::new();
    for node in nodes {
        let node_idx = ret.orderings.add_node(node);
        ret.val2index.insert(node, node_idx);
    }

    for edge in edges {
        ret.orderings.add_edge(
            *ret.val2index.get(&edge.0).unwrap(),
            *ret.val2index.get(&edge.1).unwrap(),
            (),
        );
    }

    ret
}

fn get_updates(data: &Protocol) {
    let mut sum = 0;
    //let ordered_g = petgraph::algo::toposort(&data.orderings, None).unwrap();

    for up in data.updates.iter() {
        let mut iter = up.iter();
        iter.next();
        let mut sorted = true;
        for (i, n) in iter.enumerate() {
            sorted &= data.orderings.contains_edge(
                *data.val2index.get(&up[i]).unwrap(),
                *data.val2index.get(&n).unwrap(),
            );
        }
        if !sorted {
            let mut corrected: Vec<NodeIndex> =
                up.iter().map(|x| *data.val2index.get(x).unwrap()).collect();

            corrected.sort_by(|x, y| {
                if data.orderings.contains_edge(*x, *y) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let corrected: Vec<usize> = corrected
                .iter()
                .map(|x| *data.orderings.node_weight(*x).unwrap())
                .collect();

            if !corrected.is_empty() {
                println!("- {:?}", corrected);
                sum += corrected[corrected.len() / 2];
            } else {
                println!("- Empty!!");
            }
            //println!("");
        } else {
            println!("+ {:?}", up);
        }
    }
    println!("Sum: {}", sum);
}

fn main() {
    println!("Hello, world!");
    //let data = read_input("src/example5.txt");
    let data = read_input("src/day5.txt");
    println!("Data: {:?}", &data);
    println!("");
    //println!("count: {}", test_all(&data));
    get_updates(&data);
}
