use std::error::Error;

use attendees_parser_rs::*;

fn main() {
    if let Err(err) = run() {
       eprintln!("error: {}", err);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let attendees = get_parsed_attendees("attendees.json")?;
    generate_csv("attendees.csv", attendees)?;
    Ok(())
} 
