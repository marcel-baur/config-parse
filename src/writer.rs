use crate::model::Record;
use csv::Writer;
use std::error::Error;

pub fn write(
    entries: &Vec<Record>,
    filename: String,
) -> Result<(), Box<dyn Error>> {
    let dest = generate_destination(&filename);
    let mut wtr = Writer::from_path(dest)?;
    for record in entries {
        wtr.serialize(record)?;
    }
    Ok(())
}

pub fn generate_destination(filename: &str) -> String {
    let dest_split: Vec<&str> = filename.split('.').collect();
    let mut dest: String = dest_split[0].to_string();
    dest.push_str(".csv");
    let fmt_string = dest.as_str();
    log::info!("Creating CSV: {}", fmt_string);
    dest
}
