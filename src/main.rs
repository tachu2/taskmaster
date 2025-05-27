mod readline;

fn main() {
    match readline::readline() {
        Ok(list) => {
            println!("{:?}", list);
        }
        Err(e) => {
            eprintln!("Error reading line: {}", e);
        }
    }
}
