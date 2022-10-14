use anyhow::{Context, Result};

use csv::Writer;
use generated_json_models::*;
use serde_derive::Serialize;

pub mod generated_json_models;

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
    generate_csv("attendees.csv", attendees).with_context(|| "Failed to generate csv file")?;
    Ok(())
}

fn get_parsed_attendees(file_path: &str) -> Result<Attendees> {
    let attendees: Attendees = std::fs::read_to_string(file_path)
        .with_context(|| "Failed to read json file")?
        .lines()
        .into_iter()
        .filter_map(|x| {
            serde_json::from_str::<Root>(&x)
                .ok()
                .filter(|f| f.success && !f.data.is_empty())
                .map(|f| f.data)
        })
        .flatten()
        .collect();

    Ok(attendees)
}

fn generate_csv(file_path: &str, attendees: Attendees) -> Result<()> {
    let mut wtr = Writer::from_path(file_path)?;

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
        })?;
    }
    wtr.flush()?;

    Ok(())
}
