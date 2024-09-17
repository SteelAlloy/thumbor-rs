use serde_json::Result;
use thumbor::metadata::Meta;

fn main() -> Result<()> {
    let data = include_str!("metadata.json");

    let meta: Meta = serde_json::from_str(data)?;

    dbg!(meta);
    Ok(())
}
