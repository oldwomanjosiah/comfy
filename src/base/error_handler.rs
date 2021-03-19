use colored::Colorize;

pub fn err(err: &str) {
    println!("error: {} -> --help", err.red());
    std::process::exit(0);
}

pub fn err_syntax(err: &str) {
    println!("error: {} -> exiting program", err.red());
    std::process::exit(0);
}

pub fn warning(err: &str) {
    println!("warning: {}", err.yellow());
}
