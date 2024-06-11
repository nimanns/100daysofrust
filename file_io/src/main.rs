use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    for line in reader.lines() {
        let line = line?;
        line_count += 1;
        word_count += line.split_whitespace().count();
        char_count += line.chars().count();
    }

    println!("Lines: {}", line_count);
    println!("Words: {}", word_count);
    println!("Characters: {}", char_count);

    Ok(())
}
