use std::collections::LinkedList;
use std::io::{self, Write};

pub fn readline() -> Result<LinkedList<String>, std::io::Error> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    eprint!("> ");
    io::stderr().flush()?;
    stdin.read_line(&mut buf)?;
    let list: std::collections::LinkedList<String> =
        buf.split_whitespace().map(|s| s.to_string()).collect();
    Ok(list)
}
