use std::fs;
pub fn main(args: Vec<String>) {
    dbg!(&args);
    let query = match args.get(0) {
        Some(q) => q,
        None => return,
    };
    let file_name = match args.get(1) {
        Some(q) => q,
        None => return,
    };
    let contents = fs::read_to_string(file_name).expect("shouldve been able to read");

    dbg!(contents);
}
