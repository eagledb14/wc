use std::fs;
use std::env;
use std::io;
use std::io::Read;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let arg = args[1].clone();

        let mut file = "".to_owned();

        if vec!["-c", "-l", "-w", "-m"].contains(&arg.as_str()) {
            _ = io::stdin().read_to_string(&mut file);
            run_command(&arg, &file, "");
        } else {
            file = read_file(&arg);
            println!("  {} {} {} {}", lines(&file), words(&file),  bytes(&file), arg);
        }

    } else if args.len() == 3 {
        let command = &args[1];
        let arg = &args[2];
        run_command(command, &read_file(arg), arg);
    } 
}

fn run_command(command: &str, file: &str, file_name: &str) {
    match command {
        "-c" => {
            println!("  {} {}", bytes(file), file_name);
        }
        "-l" => {
            println!("  {} {}", lines(file), file_name);
        }
        "-w" => {
            println!("  {} {}", words(file), file_name);
        }
        "-m" => {
            println!("  {} {}", chars(file), file_name);
        }
        _ => {
            panic!("nope");
        }
    }
}

fn read_file(file_name: &str) -> String{
    if let Ok(file) = fs::read_to_string(file_name) {
        return file;
    } else {
        panic!("Unknown File name: {}", file_name);
    }

}

fn bytes(file: &str) -> i32 {
    return file.as_bytes().len() as i32;
}

fn lines(file: &str) -> i32 {
    return file.split("\n").count() as i32 - 1;
}

fn words(file: &str) -> i32 {
    return file.split_whitespace().count() as i32;
}

fn chars(file: &str) -> i32 {
    return file.chars().count() as i32;
}
