#[macro_export]
macro_rules! err {
    ($err: expr) => {
        use colored::Colorize;

        eprintln!("{}: {} -> --help", "error".red(), $err);
        std::process::exit(1);
    };
}

#[macro_export]
macro_rules! err_syntax {
    ($err: expr) => {
        use colored::Colorize;

        eprintln!("{}: {} -> exiting program", "error".red(), $err);
        std::process::exit(1);
    };
}

#[macro_export]
macro_rules! warning {
    ($warn: expr) => {
        use colored::Colorize;

        eprintln!("{}: {}", "warning".yellow(), $warn);
    };
}
