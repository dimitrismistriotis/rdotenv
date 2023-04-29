use std::env;
use std::fs::File;
use std::process;

mod read_file;
use read_file::read_and_filter_file;

fn main() {
    //
    // Seems example of exec can be used as baseline
    // https://github.com/faradayio/exec-rs/blob/master/examples/exec.rs
    //
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Please specify command to wrap and execute");
        process::exit(1);
    }

    let env_file = File::open(".env");

    if env_file.is_ok() {
        set_entries_to_environment(env_file);
    } else {
        eprintln!();
        eprintln!("File .env not found, continuing wrapping without changing the environment");
        eprintln!();
    }

    // dbg!(args.clone());

    let err = exec::Command::new(&args[0]).args(&args).exec();
    println!("Error: {}", err);
    process::exit(1);
}

fn set_entries_to_environment(env_file: Result<File, std::io::Error>) {
    let filtered_lines = read_and_filter_file(env_file.unwrap());
    // println!("{}", filtered_lines.join("\n"));
    for line_with_key_value in filtered_lines {
        if let Some((variable, value)) = line_with_key_value.split_once('=') {
            // println!("{} ---> {}", variable.trim(), value.trim());
            // Add to environment:
            env::set_var(variable.trim(), value.trim());
        }
    }
}
