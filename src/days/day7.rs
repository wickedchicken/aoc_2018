use crate::file_reader::read_input;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Step {
    parent: String,
    child: String,
}

impl Step {
    fn parse(line: &str) -> Step {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Step (\w+) must be finished before step (\w+) can begin.").unwrap();
        }
        let captures = RE.captures(&line).unwrap();
        Step {
            parent: captures[1].to_string(),
            child: captures[2].to_string(),
        }
    }
}

// builds a hashmap where the key is a candidate step and the values are all
// the parent steps required before it can start. root nodes have an empty
// set for a value
fn build_tree(steps: &[Step]) -> HashMap<&String, HashSet<&String>> {
    let mut tree = HashMap::new();
    for step in steps {
        // make a blank entry for any root nodes
        tree.entry(&step.parent).or_insert_with(HashSet::new);

        let tree_entry = tree.entry(&step.child).or_insert_with(HashSet::new);
        tree_entry.insert(&step.parent);
    }

    tree
}

fn calculate_next_possible_set<'a>(
    tree: &HashMap<&'a String, HashSet<&'a String>>,
    seen: &HashSet<&'a String>,
) -> Vec<&'a String> {
    let mut candidates = Vec::new();
    for (key, value) in tree.iter() {
        if !seen.contains(key) && value.is_subset(&seen) {
            candidates.push(*key);
        }
    }
    candidates.sort_unstable();
    candidates
}

fn traverse_tree<'a>(
    tree: HashMap<&'a String, HashSet<&'a String>>,
    mut seen: HashSet<&'a String>,
) -> String {
    let res = calculate_next_possible_set(&tree, &seen);
    if res.is_empty() {
        return "".to_string();
    }
    let head = res[0];
    seen.insert(head);
    format!("{}{}", head, traverse_tree(tree, seen))
}

pub fn run() -> String {
    let stream = read_input("input/input7.txt".to_string());
    let steps: Vec<Step> = stream.map(|line| Step::parse(line.trim())).collect();
    let tree = build_tree(&steps);
    let seen = HashSet::new();
    traverse_tree(tree, seen)
}
