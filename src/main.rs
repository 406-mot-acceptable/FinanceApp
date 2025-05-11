use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    extract_tx()?;
    Ok(())
}

fn extract_tx() -> io::Result<()> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);

    for (index, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        println!("{}: {}", index + 1, line);
    }

    Ok(())
}

