mod readline;

fn main() {
    let v = readline::readline();
    match v {
        Ok(list) => {
            println!("{:?}", list);
        }
        Err(e) => {
            eprintln!("Error reading line: {}", e);
        }
    }
}
