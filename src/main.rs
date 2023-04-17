use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;


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

fn main() {
    //
    // Seems example of exec can be used as baseline
    // https://github.com/faradayio/exec-rs/blob/master/examples/exec.rs
    //
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        println!("Please specify command to wrap and execute");
        process::exit(1);
    }

    let env_file = File::open(".env");

    if env_file.is_ok() {
        let lines = lines_from_file(env_file.unwrap());
        // println!("{}", lines.join("\n"));
        let filtered_lines = filter_out_lines(lines);
        // println!("{}", filtered_lines.join("\n"));
        for line_with_key_value in filtered_lines {
            if let Some((variable, value)) = line_with_key_value.split_once('=') {
                // println!("{} ---> {}", variable.trim(), value.trim());
                // Add to environment:
                env::set_var(variable.trim(), value.trim());
            }
        }
    }

    // dbg!(args.clone());

    let err = exec::Command::new(&args[0]).args(&args).exec();
    println!("Error: {}", err);
    process::exit(1);
}
