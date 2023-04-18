use std::fs::File;
use std::io::{prelude::*, BufReader};

// https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
fn lines_from_file(file: File) -> Vec<String> {
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn filter_out_lines(all_lines: Vec<String>) -> Vec<String> {
    all_lines.into_iter()
        .filter(|line| !line.starts_with("#"))
        .filter(|line| !line.trim().is_empty())
        .collect()
}

pub fn read_and_filter_file(env_file: File) -> Vec<String> {
    let lines = lines_from_file(env_file);
    // println!("{}", lines.join("\n"));
    return filter_out_lines(lines);
}

#[cfg(test)]
mod read_file_tests {
    #[test]
    fn test_filter_out_empty_line() {
        let input_lines = vec!["".to_string()];
        let filtered_lines = super::filter_out_lines(input_lines);

        assert!(filtered_lines.len() == 0);
    }

    #[test]
    fn test_filter_out_line_line_with_multiple_spaces() {
        let input_lines = vec!["  ".to_string()];
        let filtered_lines = super::filter_out_lines(input_lines);

        assert!(filtered_lines.len() == 0);
    }

    #[test]
    fn test_filter_out_line_line_with_tabs() {
        let input_lines = vec!["\t\t  ".to_string()];
        let filtered_lines = super::filter_out_lines(input_lines);

        assert!(filtered_lines.len() == 0);
    }
}