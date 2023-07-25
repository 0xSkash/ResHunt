use std::io;

mod error;
mod model;
mod scanner;

fn main() {
    println!("Please enter the folder path to the iOS project:");
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).expect("Failed to read input.");
    let folder_path = input_path.trim();

    let collected_resources = scanner::collect_resources(folder_path);

    collected_resources
        .iter()
        .for_each(|(key, values)| {
            println!("Found {:?} of type {:?}", values.iter().count(), key)
        });
}

/*

Old Code -> gets moved into other module later

fn count_key_occurrences(folder_path: &str, keys: &[String]) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    let mut key_occurrences: HashMap<String, usize> = HashMap::new();

    let swift_files = collect_files_with_extension(folder_path, "swift");

    for swift_file in swift_files {
        let file = File::open(swift_file.path()).map_err(|_| error::Error::InvalidPath)?;
        let buffer = BufReader::new(file);

        buffer.lines()
            .filter_map(Result::ok)
            .flat_map(|line| {
                keys.iter()
                    .map(|key| (key, format_key(key)))
                    .filter(move |(_, formatted_key)| {
                        println!("{}", formatted_key);
                        return line.contains(formatted_key)
                    })
                    .map(|(original_key, _)| original_key.to_string())
            })
            .for_each(|key| {
                *key_occurrences.entry(key).or_insert(0) += 1;
            })
    }

    return Ok(key_occurrences);
}
 */