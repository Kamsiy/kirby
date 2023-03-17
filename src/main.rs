// use reqwest;
use std::fmt;
use csv::Writer;
use reqwest::blocking::Client;
use reqwest::Error;
use chrono::{Utc, Duration, NaiveDateTime};
use serde::{Serialize, Deserialize};
use table_extract::Row;

mod tests;

// Data
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    lz_houston: f32,
    lz_north: f32,
    lz_south: f32,
    lz_west: f32
}

// to_string for `Point` struct
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HOUSTON: {}  NORTH: {} SOUTH: {} WEST: {}", 
            self.lz_houston, 
            self.lz_north,
            self.lz_south,
            self.lz_west
        )
    }
}

// Returns an ercot url string with the given date
fn get_ercot_url(date: NaiveDateTime) -> String {
    let formatted_date = date.format("%Y%m%d").to_string();
    let result = format!("https://www.ercot.com/content/cdr/html/{}_dam_spp.html", formatted_date);
    result
}

// Retrieves the most recent Ercot data webpage
fn get_ercot_page() -> Result<String, Error>{
    let mut current_time = Utc::now().naive_utc();
    
    // new data is released between 16:29 UTC and 16:35 UTC
    let update_time = Utc::now().date_naive().and_hms_opt(14, 00, 0).unwrap();

    if current_time > update_time {
        // ercot url can be created with tomorrow's date 
        current_time += Duration::days(1);
    }
    let url = get_ercot_url(current_time);

    let client = Client::new();
    let response = client.get(url).send()?;

    response.text()
}

fn parse_number(header: &str, row: Row) -> f32{
    let number = row.get(header);
    let result: f32 = match number {
        Some(s) => s.parse::<f32>().unwrap(),
        None => 0.0
    };
    result
}

// Writes the Ercot data to a CSV file in the base directory called output.csv
fn export_csv(data: Vec<Point>) -> Result<(), Box<dyn std::error::Error>>{
    let mut wtr = Writer::from_path("./output.csv")?;
    
    for point in data{
        wtr.serialize(point)?;
    }

    Ok(())
}

fn parse_table(html: String) -> Vec<Point>{
    let table = table_extract::Table::find_first(html.as_str()).unwrap();
    let mut parsed_data = Vec::new();
    for row in &table {
        let curr: Point = Point {
            lz_houston: parse_number("LZ_HOUSTON", row),
            lz_south: parse_number("LZ_SOUTH", row),
            lz_west: parse_number("LZ_WEST", row),
            lz_north: parse_number("LZ_NORTH", row),
        };

        parsed_data.push(curr);
    }
    parsed_data
}

fn main() -> Result<(), Error> {

    let html = get_ercot_page()?;
    let data = parse_table(html);

    match export_csv(data) {
        Ok(()) => println!("File successfully exported to output.csv"),
        Err(error) => println!("Error: Unable to export file \n {}", error),
    }

    Ok(())
}
