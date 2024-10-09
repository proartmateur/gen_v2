use std::fs::File;
use std::io::BufReader;

use super::arq::Arq;

// Function to read a JSON file and parse it into a Vec<ArqItem>
pub fn read_arq_json(path: &str) -> Result<Arq, Box<dyn std::error::Error>> {
    // Open the file in read-only mode
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON data into a Vec<ArqItem>
    let arq_items: Arq = serde_json::from_reader(reader)?;

    Ok(arq_items)
}
