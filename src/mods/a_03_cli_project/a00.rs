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
    dbg!(file_name, query);
}
