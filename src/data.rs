//! For reading a `.csv` file

use csv::StringRecord;
use tracing::{error, trace};

/// Reads a csv of data points and returns a vector of rows
pub fn load_data(data_path: &str, has_headers: bool, csv_delimiter: &str) -> Vec<StringRecord> {
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
	let mut string_records: Vec<StringRecord> = Vec::new();
	for record in data.records() {
		match record {
			Ok(r) => {
				trace!("Csv data: {:?}", r);
				string_records.push(r)
			}
			Err(e) => {
				error!("Unable to read record in csv data: {}", e);
				std::process::exit(1);
			}
		}
	}
	return string_records;
}
