use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    
    let filepath = env::args()
        .nth(1)
        .expect("usage: cargo run -- <file-path>");

    extract_tx(&filepath)?;
    Ok(())
}

fn extract_tx(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (index, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        println!("{}: {}", index + 1, line);
    }

    Ok(())
}

