use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::{Directed, Graph, Incoming};
use petgraph::algo::{toposort, DfsSpace};
use petgraph::data::{Build, DataMap};
use petgraph::graph::{NodeIndex};
use petgraph::visit::NodeIndexable;

#[derive(Debug)]
struct Protocol {
    orderings: Graph<usize, (), Directed>,
    updates: Vec<Vec<usize>>,
    val2index: HashMap<usize, NodeIndex>,
}

fn read_input(file:  &str) -> Protocol {
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
            if line == "" { mode = false; continue; }
            if mode {
                let (a, b) = line.split_once("|")
                    .ok_or(format!("Invalid line: {}", line))
                    .and_then(|(a, b)| {
                        Ok((a.parse::<usize>().unwrap(),
                            b.parse::<usize>().unwrap()))
                    }).unwrap();
                println!("{}: ({} {})", i+1, a, b);
                edges.push((a,b));
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
        let nodeIdx = ret.orderings.add_node(node);
        ret.val2index.insert(node, nodeIdx);
    }

    for edge in edges {
        ret.orderings.add_edge(
            *ret.val2index.get(&edge.0).unwrap(),
            *ret.val2index.get(&edge.1).unwrap(),
            ());
    }

    ret
}

fn get_updates(data: &Protocol) {
    let mut sum = 0;
    let sorted_g = toposort(&data.orderings, None).unwrap();
    for up in data.updates.iter() {
        let mut iter = up.iter();
        iter.next();
        let mut contains = true;
        for (i, n) in iter.enumerate() {
            //println!("{}: {}", up[i], n);
            contains &= data.orderings.contains_edge(
                *data.val2index.get(&up[i]).unwrap(),
                *data.val2index.get(&n).unwrap()
            );
        }
        if !contains {
            //println!("- {:?}", up);
            //TODO: bring up in correct order
            let mut corrected = vec![];
            for ni in sorted_g.iter() {
                let nw = data.orderings.node_weight(*ni).unwrap();
                if up.contains(nw) {
                    print!("{:?} ", nw);
                    corrected.push(*nw);
                }
            }
            sum += corrected[corrected.len()/2];
            println!("");
        } else {
            println!("+ {:?}", up);
        }
    }
    println!("Sum: {}", sum);
}

fn main() {
    println!("Hello, world!");
    let data = read_input("src/example5.txt");
    //let data = read_input("src/day5.txt");
    println!("Data: {:?}", &data);
    println!("");
    //println!("count: {}", test_all(&data));
    get_updates(&data);
}