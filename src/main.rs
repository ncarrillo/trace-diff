use std::{env, fs::File, io::{BufRead, BufReader}};

fn find_first_difference(file1_path: &str, file2_path: &str) -> Result<(), String> {
    let file1 = BufReader::new(File::open(file1_path).unwrap());
    let file2 = BufReader::new(File::open(file2_path).unwrap());

    let mut line_buffer1 = Vec::new();
    let mut line_number = 0;

    println!("");
    println!("comparing traces");
    println!("");

    for (line1, line2) in file1.lines().zip(file2.lines()) {
        line_number += 1;

        let line1 = line1.unwrap();
        let line2 = line2.unwrap();

        if line1.trim() != line2.trim() {
            // capture the last 5 lines before the difference
            let start_context = if line_number > 5 { line_number - 5 } else { 0 };
            let last_lines1: Vec<String> = line_buffer1.iter().skip(start_context).cloned().collect();

            // print context lines
            for ctx in last_lines1 {
                println!("{}", ctx);
            }

            // print the difference then early exit
            println!("");
            println!("Found a difference in the trace @ {}:", line_number);
            println!("");
            println!("reference:");
            println!("{}", line1);
            println!("");
            println!("emerald:");
            println!("{}", line2);
            println!("");

            return Err(format!("difference detected @ {}", line_number));
        }

        // keep track of context in the line buffer in case there's a difference.
        line_buffer1.push(line1);
    }

    println!("well done :-). validated {} cycles.", line_number);
    return Ok(());
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("invalid argument length".into());
    }

    let file1_path = &args[1];
    let file2_path = &args[2];

    find_first_difference(&file1_path, &file2_path)
}