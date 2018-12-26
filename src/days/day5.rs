use crate::file_reader::read_input;
use unicode_segmentation::UnicodeSegmentation;

fn process_string(input: &[String]) -> Vec<String> {
    let mut output: Vec<usize> = Vec::with_capacity(input.len());
    let mut input_ints: Vec<usize> = (0..(input.len())).collect();
    let input_cap: Vec<String> = input.iter().map(|x| x.to_uppercase().to_string()).collect();
    loop {
        let mut last_char: Option<usize> = None;
        let mut modified = false;
        for idx in input_ints.iter() {
            match last_char {
                None => {
                    last_char = Some(*idx);
                }
                Some(last_char_idx) => {
                    let grapheme = &input[*idx];
                    let grapheme_cap = &input_cap[*idx];
                    let last_char_grapheme = &input[last_char_idx];
                    let last_char_grapheme_cap = &input_cap[last_char_idx];
                    if (grapheme != last_char_grapheme) && (grapheme_cap == last_char_grapheme_cap)
                    {
                        modified = true;
                        last_char = None;
                    } else {
                        output.push(last_char_idx);
                        last_char = Some(*idx);
                    }
                }
            }
        }
        match last_char {
            None => (),
            Some(last_char_idx) => {
                output.push(last_char_idx);
            }
        }

        if !modified {
            return output.iter().map(|x| input[*x].clone()).collect();
        }

        input_ints = output;
        output = Vec::with_capacity(input.len());
    }
}

pub fn run() -> usize {
    let mut final_res = 0;
    for line in read_input("input/input5.txt".to_string()) {
        let graphemes: Vec<String> = UnicodeSegmentation::graphemes(line.trim(), true)
            .map(|x| x.to_string())
            .collect();
        let res = process_string(&graphemes);
        final_res += res.len();
    }
    final_res
}
