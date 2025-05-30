mod commandline;

fn main() {
    match commandline::CommandLine::readline() {
        Ok(line) => {
            println!("{:?}", line);
        }
        Err(e) => {
            eprintln!("Error reading line: {}", e);
        }
    }
}
