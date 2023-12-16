use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder, Trim};
use serde::Deserialize;

// Struct to represent a route with 'from' and 'to' node IDs
#[derive(Debug, Deserialize, Clone)]
pub struct AirportRoute {
    pub from: u32,  
    pub to: u32,
}

pub fn read_file(file_path: &str) -> Result<Vec<AirportRoute>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .trim(Trim::All)
        .from_reader(file);

    let mut routes = Vec::new();

    for result in csv_reader.deserialize() {
        let route: AirportRoute = result?;
        routes.push(route);
    }

    Ok(routes)
}
