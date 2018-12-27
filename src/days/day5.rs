use crate::file_reader::read_input;
use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn process_string(input: &[String]) -> Vec<String> {
    let mut output: Vec<String> = Vec::with_capacity(input.len());
    for grapheme in input {
        let last_char = output.pop();
        if last_char.is_none() {
            output.push(grapheme.to_string());
            continue;
        }

        let last_char = last_char.unwrap();
        if (*grapheme == last_char) || (grapheme.to_uppercase() != last_char.to_uppercase()) {
            output.push(last_char);
            output.push(grapheme.to_string());
        }
    }
    output
}

fn get_letters(input: &[String]) -> HashSet<String> {
    let mut seen_letters = HashSet::new();
    for grapheme in input.iter() {
        seen_letters.insert(grapheme.to_lowercase());
    }
    seen_letters
}

fn make_string_without(input: &[String], remove: &str) -> Vec<String> {
    let remove = remove.to_lowercase();
    let mut output = Vec::with_capacity(input.len());
    for grapheme in input.iter() {
        if grapheme.to_lowercase() != remove {
            output.push(grapheme.clone());
        }
    }
    output
}

pub fn run() -> (usize, usize) {
    let mut final_res = 0;
    let mut smaller_res = 0;
    for line in read_input("input/input5.txt".to_string()) {
        let graphemes: Vec<String> = UnicodeSegmentation::graphemes(line.trim(), true)
            .map(|x| x.to_string())
            .collect();
        let res = process_string(&graphemes);
        final_res += res.len();

        let letters = get_letters(&graphemes);
        smaller_res += letters
            .iter()
            .map(|x| process_string(&make_string_without(&graphemes, x)).len())
            .min()
            .unwrap();
    }
    (final_res, smaller_res)
}
