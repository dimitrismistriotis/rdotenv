use std::env;
use std::process;

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

    dbg!(args.clone());

    let err = exec::Command::new(&args[0]).args(&args).exec();
    println!("Error: {}", err);
    process::exit(1);
}
