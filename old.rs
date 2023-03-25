fn main() {
let file = File::open("calendar.csv").unwrap();
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(reader);

    for result in csv_reader.records() {
        let record = result.unwrap();
        println!("{}", record.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "));
    }
}