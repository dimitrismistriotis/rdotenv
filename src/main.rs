use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let joined_args = args.join(" ");
    println!("{}", joined_args);

    let err = exec::Command::new("echo")
        .arg("hello").arg("world")
        .exec();
    println!("Error: {}", err);
}
