use crate::prelude::*;
use mtg_core::cache::{CachedHttpClient, DiskCacheBuilder};
use mtg_core::{ContentfulItemWithId, ContentfulResponse, RankedDecksClient, RankedListParams};
use prettytable::{Cell, Row};

pub async fn run(
    format_filter: Option<String>,
    limit: u32,
    skip: u32,
    output: String,
    global: crate::Global,
) -> Result<()> {
    // Create cache and HTTP client
    let cache = DiskCacheBuilder::new().prefix("ranked_list").build()?;

    let http_client = CachedHttpClient::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .cache_prefix("ranked_list_http")
        .build()?;

    // Create ranked decks client
    let client = RankedDecksClient::new(http_client, cache);

    // Create parameters
    let params = RankedListParams {
        format_filter: format_filter.clone(),
        limit,
        skip,
    };

    // Fetch ranked list using mtg_core
    let response = client.fetch_ranked_list(params).await?;

    // Output results
    match output.as_str() {
        "json" => {
            output_ranked_json_with_ids(&response.contentful_response, &response.items_with_ids)?
        }
        "pretty" => output_ranked_pretty_with_ids(
            &response.items_with_ids,
            response.contentful_response.total,
            response.contentful_response.skip,
            format_filter,
            limit,
        )?,
        _ => output_ranked_pretty_with_ids(
            &response.items_with_ids,
            response.contentful_response.total,
            response.contentful_response.skip,
            format_filter,
            limit,
        )?,
    }

    Ok(())
}

fn output_ranked_json_with_ids(
    response: &ContentfulResponse,
    items_with_ids: &[ContentfulItemWithId],
) -> Result<()> {
    let output = serde_json::json!({
        "sys": response.sys,
        "total": response.total,
        "skip": response.skip,
        "limit": response.limit,
        "items": items_with_ids.iter().map(|item_with_id| {
            let mut item_json = serde_json::to_value(&item_with_id.item).unwrap();
            if let Some(obj) = item_json.as_object_mut() {
                obj.insert("id".to_string(), serde_json::Value::String(item_with_id.id.clone()));
            }
            item_json
        }).collect::<Vec<_>>()
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn output_ranked_pretty_with_ids(
    items_with_ids: &[ContentfulItemWithId],
    total: u32,
    skip: u32,
    format_filter: Option<String>,
    limit: u32,
) -> Result<()> {
    if items_with_ids.is_empty() {
        println!("No deck lists found.");
        return Ok(());
    }

    // Create a table for the deck lists
    let mut table = new_table();
    table.add_row(Row::new(vec![
        Cell::new("Id"),
        Cell::new("Title"),
        Cell::new("Author"),
        Cell::new("Published"),
        Cell::new("Link"),
    ]));

    for item_with_id in items_with_ids {
        let item = &item_with_id.item;
        let published_date = item
            .fields
            .published_date
            .split('T')
            .next()
            .unwrap_or(&item.fields.published_date);

        let author = item.fields.author.as_deref().unwrap_or("Unknown");

        // Construct the link from the slug if outboundLink is not available
        let link = if let Some(outbound) = &item.fields.outbound_link {
            outbound.clone()
        } else {
            format!("https://magic.gg/decklists/{}", item.fields.slug)
        };

        table.add_row(Row::new(vec![
            Cell::new(&item_with_id.id),
            Cell::new(&item.fields.article_title),
            Cell::new(author),
            Cell::new(published_date),
            Cell::new(&link),
        ]));
    }

    table.printstd();
    println!();

    if let Some(ref format) = format_filter {
        println!("Format: {}", format.to_uppercase());
    }
    println!("Total Results: {}", total);
    println!(
        "Showing: {} - {}\n",
        skip + 1,
        skip + items_with_ids.len() as u32
    );

    // Display pagination commands
    let current_page = (skip / limit) + 1;
    let total_pages = total.div_ceil(limit);

    if total_pages > 1 {
        aeprintln!("Page {} of {}", current_page, total_pages);

        // Build base command
        let mut base_cmd = String::from("mtg decks ranked list");
        if let Some(fmt) = &format_filter {
            base_cmd.push_str(&format!(" --format {}", fmt));
        }
        if limit != 20 {
            base_cmd.push_str(&format!(" --limit {}", limit));
        }

        // Show navigation commands
        if current_page > 1 {
            aeprintln!("Previous page: {} --page {}", base_cmd, current_page - 1);
        }
        if current_page < total_pages {
            aeprintln!("Next page: {} --page {}", base_cmd, current_page + 1);
        }
        if current_page != 1 {
            aeprintln!("First page: {} --page 1", base_cmd);
        }
        if current_page != total_pages {
            aeprintln!("Last page: {} --page {}", base_cmd, total_pages);
        }
    }

    Ok(())
}
