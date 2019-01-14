use crate::file_reader::read_input;
use aho_corasick::{AcAutomaton, Automaton, Match};
use regex::Regex;

#[derive(Clone, Debug)]
struct Rule {
    conditions: String,
    result: char,
}

impl Rule {
    fn parse(line: &str) -> Option<Rule> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([\.#]+) => ([\.#])").unwrap();
        }
        if let Some(captures) = RE.captures(&line) {
            Some(Rule {
                conditions: captures[1].to_string(),
                result: captures[2].chars().next().unwrap(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct State {
    data: String,
}

impl State {
    fn parse(line: &str) -> Option<State> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"initial state: ([\.#]+)").unwrap();
        }
        if let Some(captures) = RE.captures(&line) {
            Some(State {
                data: captures[1].to_string(),
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct RuleAutomaton {
    rules: Vec<Rule>,
    automaton: AcAutomaton<String>,
}

impl RuleAutomaton {
    fn new(rules: Vec<Rule>) -> RuleAutomaton {
        let rule_strings = rules
            .iter()
            .map(|x| x.conditions.clone())
            .collect::<Vec<String>>();
        let automaton = AcAutomaton::new(rule_strings);
        RuleAutomaton { rules, automaton }
    }

    fn return_matches(&self, state: &str) -> Vec<(usize, char)> {
        let mut hits = self
            .automaton
            .find_overlapping(state)
            .collect::<Vec<Match>>();
        hits.sort_unstable_by_key(|k| k.start);
        hits.iter()
            .map(|x| (x.start + 2, self.rules[x.pati].result))
            .collect()
    }
}

#[derive(Debug)]
struct Model {
    rule_automaton: RuleAutomaton,
    state: State,
}

impl Iterator for Model {
    type Item = Model;

    fn next(&mut self) -> Option<Model> {
        let new_str = format!("....{}....", &self.state.data);
        let matches = self.rule_automaton.return_matches(&new_str);

        let mut process_str = String::with_capacity(new_str.len());
        let mut matches_itr = matches.iter();
        let mut next_match = matches_itr.next();
        let mut modified = false;

        for i in 0..new_str.len() {
            if let Some(m) = next_match {
                if i == m.0 {
                    process_str.push(m.1);
                    next_match = matches_itr.next();
                    modified = true;
                } else {
                    process_str.push('.')
                }
            } else {
                process_str.push('.')
            }
        }

        if modified {
            self.state.data = process_str.clone();
            Some(Model {
                rule_automaton: self.rule_automaton.clone(),
                state: State { data: process_str },
            })
        } else {
            None
        }
    }
}

pub fn run() -> i32 {
    let mut rules = Vec::new();
    let mut initial_state: Option<State> = None;

    for line in read_input("input/input12.txt".to_string()) {
        if let Some(new_initial_state) = State::parse(line.trim()) {
            initial_state = Some(new_initial_state);
        } else if let Some(rule) = Rule::parse(line.trim()) {
            rules.push(rule);
        }
    }

    let model = Model {
        state: initial_state.unwrap(),
        rule_automaton: RuleAutomaton::new(rules),
    };

    let iters = 20;
    let offset = iters * 4;

    model
        .take(iters as usize)
        .last()
        .unwrap()
        .state
        .data
        .match_indices('#')
        .map(|x| (x.0 as i32) - offset)
        .sum()
}
