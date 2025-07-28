use crate::prelude::*;

pub async fn run(query: &str, include_extras: bool, global: crate::Global) -> Result<()> {
    if global.verbose {
        println!("Getting autocomplete suggestions for: {query}");
    }

    // Create Scryfall client using the global client
    let scryfall_client = global.create_scryfall_client()?;

    // Get autocomplete suggestions
    let autocomplete = scryfall_client
        .autocomplete(query, Some(include_extras))
        .await?;

    // Display suggestions
    for suggestion in &autocomplete.data {
        println!("{suggestion}");
    }

    if global.verbose {
        aeprintln!("Found {} suggestions", autocomplete.total_values);
    }

    Ok(())
}
