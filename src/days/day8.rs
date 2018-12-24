extern crate unicode_segmentation;
use crate::file_reader::read_input;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
struct Node {
    children_idx: Vec<usize>,
    metadata: Vec<i32>,
}

impl Node {
    fn parse(stream: &mut Iterator<Item = &str>, nodes: &mut Vec<Node>) -> usize {
        let child_node_count: i32 = stream.next().unwrap().parse().unwrap();
        let metadata_count: i32 = stream.next().unwrap().parse().unwrap();
        let mut children_idx = Vec::new();
        for _ in 0..child_node_count {
            children_idx.push(Node::parse(stream, nodes));
        }
        let location = nodes.len();
        let mut metadata = Vec::new();
        for _ in 0..metadata_count {
            metadata.push(stream.next().unwrap().parse().unwrap());
        }
        nodes.push(Node {
            children_idx,
            metadata,
        });
        location
    }
}

pub fn run() -> i32 {
    let mut nodes = Vec::new();
    for line in read_input("input/input8.txt".to_string()) {
        let mut words = line.trim().unicode_words();
        Node::parse(&mut words, &mut nodes);
    }
    nodes.iter().map(|x| x.metadata.iter().sum::<i32>()).sum()
}
