//!

use std::fs::File;

use csv::Reader;
use tracing::{error, trace};

/// Reads a csv of data points
pub fn load_data(data_path: &str, has_headers: bool, csv_delimiter: &str) -> Reader<File> {
	let mut data = match csv::ReaderBuilder::new()
		.has_headers(has_headers)
		.delimiter(csv_delimiter.as_bytes()[0])
		.from_path(data_path)
	{
		Ok(d) => d,
		Err(e) => {
			error!("Unable to read csv data {:?}", e);
			std::process::exit(1);
		}
	};
	for record in data.records() {
		trace!("Csv data: {:?}", record);
	}
	return data;
}
