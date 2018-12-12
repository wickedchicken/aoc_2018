use file_reader::read_input;
use unicode_segmentation::UnicodeSegmentation;

fn process_string(input: Vec<&str>) -> Vec<&str> {
    let mut last_char: Option<&str> = None;
    let mut modified = false;
    let mut output = Vec::new();
    for grapheme in input {
        match last_char {
            None => {
                last_char = Some(grapheme);
            }
            Some(g) => {
                if (g != grapheme) && (g == grapheme.to_uppercase()) {
                    modified = true;
                    last_char = None;
                } else if (g != grapheme) && (g.to_uppercase() == grapheme) {
                    modified = true;
                    last_char = None;
                } else {
                    output.push(g);
                    last_char = Some(grapheme);
                }
            }
        }
    }
    match last_char {
        None => (),
        Some(g) => output.push(g),
    }

    if modified {
        return process_string(output);
    }

    return output;
}

pub fn run() -> usize {
    for line in read_input("input/input5.txt".to_string()) {
        let graphemes = UnicodeSegmentation::graphemes(line.trim(), true).collect();
        let res = process_string(graphemes);
        return res.len();
    }
    panic!("No input detected!");
}
