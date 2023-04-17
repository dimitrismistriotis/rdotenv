use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};
use std::process;


// https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
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


    let lines = lines_from_file(".env");
    println!("{}", lines.join(", "));

    //
    // Add to environment:
    //
    env::set_var("TEST_ENVIRONMENT_VARIABLE", format!("ARGS_LENGTH_IS {}", args.len()));

    dbg!(args.clone());

    let err = exec::Command::new(&args[0]).args(&args).exec();
    println!("Error: {}", err);
    process::exit(1);
}
