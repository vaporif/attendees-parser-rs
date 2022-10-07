use anyhow::{Context, Result};
use attendees_parser_rs::*;

fn main() -> Result<()> {
    run().with_context(|| format!("Program failed"))?;

    Ok(())
}
