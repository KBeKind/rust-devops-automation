use std::io::{BufReader, Read};
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let path = Path::new(&args[1]);

    if !path.exists() {
        println!("Path {} does not exist", path.display());
        return;
    }

    walk_path(path);
}
