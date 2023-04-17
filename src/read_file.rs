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
        .filter(|line| !line.is_empty())
        .collect()
}

pub fn read_and_filter_file(env_file: File) -> Vec<String> {
    let lines = lines_from_file(env_file);
    // println!("{}", lines.join("\n"));
    return filter_out_lines(lines);
}