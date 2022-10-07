use std::{
    fs::{self},
    io::{self, BufRead},
};

use csv::Writer;
use generated_json_models::*;
use serde_derive::Serialize;

pub mod errors;
pub mod generated_json_models;

use errors::*;

#[macro_use]
extern crate error_chain;

type Attendee = Daum;
type Attendees = Vec<Attendee>;

#[derive(Debug, Serialize)]
struct Record {
    attendee_type: String,
    company_name: String,
    headline: String,
    summary: Option<String>,
    location: String,
    job_title: String,
    name: String,
    picture_url: Option<String>,
}

pub fn run() -> Result<()> {
    let attendees = get_parsed_attendees("attendees.json")?;
    generate_csv("attendees.csv", attendees)?;
    Ok(())
}

fn get_parsed_attendees(file_path: &str) -> Result<Attendees> {
    let file = fs::File::open(file_path).chain_err(|| "unable to open file")?;

    // note: i know it's inefficient and simple fs::read_lines would be better
    let attendees: Attendees = io::BufReader::new(file)
        .lines()
        .into_iter()
        .filter_map(|x| {
            x.chain_err(|| "line read failure").ok().and_then(|res| {
                serde_json::from_str::<Root>(&res)
                    .chain_err(|| "parse failure")
                    .ok()
                    .filter(|f| f.success && !f.data.is_empty())
                    .map(|f| f.data)
            })
        })
        .flatten()
        .collect();

    Ok(attendees)
}

fn generate_csv(file_path: &str, attendees: Attendees) -> Result<()> {
    let mut wtr = Writer::from_path(file_path).chain_err(|| "unable to open csv file")?;

    for attendee in attendees.into_iter() {
        wtr.serialize(Record {
            attendee_type: attendee.type_key_translation,
            company_name: attendee.company_name,
            headline: attendee.headline,
            summary: attendee.summary,
            location: attendee.location,
            job_title: attendee.job_title,
            name: attendee.name,
            picture_url: attendee.picture_url,
        })
        .chain_err(|| "unable to write csv row")?;
    }
    wtr.flush().chain_err(|| "unable to save csv file")?;

    Ok(())
}
