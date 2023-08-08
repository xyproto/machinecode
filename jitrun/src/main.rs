use machinecode::{execute, string_to_bytes};
use std::fs;
use std::io::{self, Read};
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide the filename of a hex encoded machine code program as the first argument.");
        eprintln!("Use -s to silence the output and only set the exit code to the return code of the program.");
        process::exit(1);
    }

    let mut verbose = true;
    let filename = if args[1] == "-s" || args[1] == "--silent" {
        if args.len() < 3 {
            eprintln!("Please provide the filename of a hex encoded machine code program as the second argument when using -s or --silent.");
            process::exit(1);
        }
        verbose = false;
        &args[2]
    } else {
        &args[1]
    };

    let data = if filename == "-" {
        let mut data = String::new();
        io::stdin()
            .read_to_string(&mut data)
            .expect("Failed to read from stdin");
        data
    } else {
        fs::read_to_string(filename).expect("Failed to read the file")
    };

    let source_code: String = data
        .lines()
        .map(|line| {
            let pos = line
                .find(|c| c == ';' || c == '#' || c == '/')
                .unwrap_or(line.len());
            line[..pos].trim()
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    if verbose {
        println!("Stripped source code:\n{}\n", source_code);
    }

    let code = string_to_bytes(&source_code).expect("Failed to convert string to bytes");

    if verbose {
        println!("Source bytes:\n{:?}\n", code);
    }

    match execute(&code) {
        Ok(retval) => {
            if verbose {
                println!("The program returned: {}", retval);
            } else {
                process::exit(retval);
            }
        }
        Err(e) => {
            panic!("Error executing the code: {}", e);
        }
    }
}
