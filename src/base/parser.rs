use crate::base::error_handler::{err, err_syntax, warning};
use colored::Colorize;
use std::{
    env::consts,
    ffi::OsStr,
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::Path,
    process::Command,
    thread, time,
};

pub fn parse(file: &str, show_comments: bool) {
    let os = consts::OS;
    let pattern = ">";

    if check_file(file) {
        let parse_file = File::open(file).unwrap();
        let parse_reader = BufReader::new(parse_file);
        let mut line_os: String = "default".to_string();

        for (index, line) in parse_reader.lines().enumerate() {
            let line = line.unwrap();
            let argument: Vec<&str> = line.split_whitespace().collect();

            if !line.is_empty() {
                if !(argument[0] == "//" || line.trim().is_empty() || argument[0] == "@") {
                    if argument[0] == pattern {
                        line_os = argument[1].to_string();
                    } else if line_os == os || line_os == "always" {
                        exe_line(index, &line, os);
                    }
                } else if show_comments {
                    print_line(index, &line, "sys");
                } else if kword(&line, index) {
                    // kword does everything
                } else if !(&line[..2] == "//") {
                    warning(
                        &(("syntax, line ".to_owned() + &(index + 1).to_string())
                            + &" -> ".to_owned()
                            + &line
                            + &" parser does not recognize it".to_owned()),
                    );
                }
            } else {
                warning(
                    &(("syntax, line ".to_owned() + &(index + 1).to_string())
                        + &" -> ".to_owned()
                        + &"blank lines can originate errors".to_owned()),
                );
            }
        }
    }
}

fn kword(line: &str, index: usize) -> bool {
    let argument: Vec<&str> = line.split_whitespace().collect();
    if argument[0] == "@" {
        match argument[1] {
            "sleep" => {
                print_line(index, &line, "non");
                if !argument[2].chars().all(char::is_numeric) {
                    err_syntax(
                        &(("syntax error, line ".to_owned() + &(index + 1).to_string())
                            + &" -> ".to_owned()
                            + argument[2]
                            + &" is not [int]".to_owned()),
                    );
                }
                thread::sleep(time::Duration::from_millis(
                    (argument[2]).parse::<u64>().unwrap(),
                ));
                return true;
            }
            _ => {
                err_syntax(
                    &(("syntax error, line ".to_owned() + &(index + 1).to_string())
                        + &" -> ".to_owned()
                        + argument[1]
                        + &" is not a comfy function".to_owned()),
                );
                return true;
            }
        }
    } else {
        return false;
    }
}

fn check_file(file: &str) -> bool {
    if Path::new(file).is_file() && Path::new(file).extension() == Some(OsStr::new("comfy")) {
        println!("script {} found", file);
        return true;
    } else if Path::new(file).is_file() {
        println!("{} is not a .comfy file, proceed? (y/N)", file);
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim_end().to_lowercase() == "y" {
                    return true;
                } else {
                    return false;
                }
            }
            Err(e) => {
                err(&e.to_string());
                return false;
            }
        }
    } else {
        err_syntax(&("no such file named ".to_string() + file));
        return false;
    }
}

fn exe_line(index: usize, line: &str, os: &str) {
    print_line(index, &line, "sys");
    if os == "windows" {
        Command::new("cmd")
            .args(&["/C", &line])
            .status()
            .expect(&("err, line -> ".to_owned() + &index.to_string()));
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&line)
            .status()
            .expect(&("system err, line -> ".to_owned() + &index.to_string()));
    };
}

fn print_line(index: usize, line: &str, i: &str) {
    if i == "sys" {
        println!(
            "{}{} {}",
            (index + 1).to_string().truecolor(150, 150, 150),
            ":".truecolor(150, 150, 150),
            line.truecolor(150, 150, 150)
        );
    } else if i == "non" {
        // changable color
        println!(
            "{}{} {}",
            (index + 1).to_string().truecolor(150, 150, 150),
            ":".truecolor(150, 150, 150),
            line.truecolor(150, 150, 150)
        );
    }
}
