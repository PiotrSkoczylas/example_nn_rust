use crate::modules::iris;
use csv;
use std::error::Error;

pub fn read_from_file(path: &str) -> Result<Vec<iris::Iris>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut dataset = Vec::new();
    for result in reader.deserialize() {
        let record: iris::Iris = result?;
        dataset.push(record.clone());
    }
    Ok(dataset)
}
