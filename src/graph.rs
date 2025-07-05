use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;
use csv::Reader;
use crate::location::Location;

pub type Graph = HashMap<Arc<Location>, HashMap<Arc<Location>, i32>>;


pub fn construct_graph(mut reader: Reader<File>) -> Result<Graph, csv::Error> {

	let mut graph: Graph = HashMap::new();
	for result in reader.records() {
		match result {
			Ok(record) => {
				if record.len() != 5 {
					panic!("Invalid record length: expected 5 fields, got {}", record.len());
				}

				let start_city = record[0].to_string();
				let start_country = record[1].to_string();
				let end_city = record[2].to_string();
				let end_country = record[3].to_string();
				let distance: i32 = match record[4].parse() {
					Ok(d) => d,
					Err(_) => continue,
				};

				let start_location = crate::location::Location {
					city: start_city,
					country: start_country,
				};
				let end_location = crate::location::Location {
					city: end_city,
					country: end_country,
				};

				let start_location = Arc::new(start_location);
				let end_location = Arc::new(end_location);


				graph.entry(start_location.clone()).or_insert(HashMap::new()).insert(end_location.clone(), distance);
				graph.entry(end_location.clone()).or_insert(HashMap::new()).insert(start_location.clone(), distance);

			},
			Err(e) => eprintln!("Error reading record: {}", e),
		}
	}

	Ok(graph)

}


