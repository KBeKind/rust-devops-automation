use flate2::read::GzDecoder;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::{env, fs};

fn walk_path(path: &Path) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        //println!("Entry Path: {}", entry.path().display());

        let path = entry.path();

        if path.is_dir() {
            walk_path(&path);
        } else {
            let file = fs::File::open(&path).unwrap();
            let mut buffer = [0; 1024];
            let mut reader = BufReader::new(file);
            let bytes_read = reader.read(&mut buffer).unwrap();

            if bytes_read > 0 && std::str::from_utf8(&buffer[..bytes_read]).is_ok() {
                println!("Plain text file: {}", path.display());
            } else {
                println!("Non-Plain text file: {}", path.display());
            }

            //println!("{}", path.display());
        }
    }
}

fn read_buffer(file_path: &str) {
    // INITIALIZE VARIABLES FOR ERROR RATE CALCULATION
    let mut total_entries = 0;
    let mut error_entries = 0;
    let mut current_hour = None;

    let timestamp_regex = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}-\d{2})").unwrap();
    let error_keyword = "Error";
    let file = File::open(file_path).unwrap();

    let reader: Box<dyn BufRead> = match file_path.ends_with(".gz") {
        true => {
            // DECOMPRESS GZIPPED FILE
            let decompressor = GzDecoder::new(file);
            Box::new(BufReader::new(decompressor))
        }
        false => Box::new(BufReader::new(file)),
    };

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                continue;
            }
        };

        // EXTRACT TIMESTAMP FROM THE LOG LINE

        if let Some(captures) = timestamp_regex.captures(&line) {
            let timestamp = &captures[1];

            // EXTRACT THE DATE AND HOUR PART OF THE TIMESTAMP
            let date = &timestamp[0..10];
            let hour = &timestamp[11..13];
            let date_hour = format!("{}, Hour: {}", date, hour);

            // CHECK IF THE CURRENT HOUR IS DIFFERENT FROM THE PREVIOUS HOUR
            if current_hour != Some(date_hour.to_string()) {}
            {
                // CALCULATE AND PRINT ERROR COUNT FOR THE PREVIOUS HOUR
                if let Some(prev_hour) = current_hour.take() {
                    println!("{prev_hour} - Error Count: {error_entries}");
                }

                // RESET COUNTERS FOR THE NEW HOUR
                error_entries = 0;
                current_hour = Some(date_hour.to_string());
            }

            // CHECK IF THE LOG ENTRY CONTAINS AN ERROR
            if line.contains(error_keyword) {
                error_entries += 1;
                total_entries += 1;
            }
        }
    }

    println!("Total error count for the current log: {total_entries}");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: log_error_rate <log_file_path>");
        std::process::exit(1);
    }

    let log_file_path = &args[1];

    read_buffer(log_file_path);

    //let path = Path::new(&args[1]);

    // if !path.exists() {
    //     println!("Path {} does not exist", path.display());
    //     return;
    // }

    // walk_path(path);
}
