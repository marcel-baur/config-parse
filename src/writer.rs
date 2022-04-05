use crate::model::Record;
use csv::Writer;
use std::error::Error;

pub fn write(entries: Vec<Record>, dest: String) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(dest)?;
    for record in entries {
        println!("{:?}", record);
        wtr.serialize(record)?;
    }

    Ok(())
}
