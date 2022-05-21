use std::fs::File;
use std::io::Write;
fn main() {
    let mut file = File::create("data.csv").unwrap();
    //let s = "test,string".to_string();
    writeln!(file, "{}, {}", 123, 456).unwrap();
    //file.write_all(s.as_bytes()).unwrap();
}
