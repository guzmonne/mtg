use crate::prelude::*;
use prettytable::{Cell, Row};
use serde::{Deserialize, Serialize};

// Extended item with ID for display
#[derive(Debug, Clone)]
pub struct ContentfulItemWithId {
    pub id: String,
    pub item: ContentfulItem,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentfulFields {
    #[serde(rename = "metaTitle")]
    pub meta_title: Option<String>,
    #[serde(rename = "metaDescription")]
    pub meta_description: Option<String>,
    #[serde(rename = "articleTitle")]
    pub article_title: String,
    #[serde(rename = "outboundLink")]
    pub outbound_link: Option<String>,
    pub slug: String,
    pub author: Option<String>,
    #[serde(rename = "publishedDate")]
    pub published_date: String,
    #[serde(rename = "decklistBody")]
    pub decklist_body: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentfulItemSys {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentfulItem {
    pub sys: ContentfulItemSys,
    pub fields: ContentfulFields,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContentfulSys {
    #[serde(rename = "type")]
    pub sys_type: String,
}

// Contentful API response structures
#[derive(Debug, Deserialize, Serialize)]
pub struct ContentfulResponse {
    pub sys: ContentfulSys,
    pub total: u32,
    pub skip: u32,
    pub limit: u32,
    pub items: Vec<ContentfulItem>,
}

pub async fn run(
    format_filter: Option<String>,
    limit: u32,
    skip: u32,
    output: String,
    global: crate::Global,
) -> Result<()> {
    // Build the URL with query parameters
    let mut url = format!(
        "https://cdn.contentful.com/spaces/ryplwhabvmmk/environments/master/entries?content_type=decklistArticle&include=10&order=-fields.publishedDate&limit={}&skip={}",
        limit, skip
    );

    // Add format filter if provided
    if let Some(format) = &format_filter {
        url.push_str(&format!("&fields.articleTitle%5Bmatch%5D={}", format));
    }

    // Create HTTP client
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(global.timeout))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .build()?;

    // Make the request
    let response = client
        .get(&url)
        .header("accept", "application/json, text/plain, */*")
        .header("accept-language", "en-US,en;q=0.9")
        .header(
            "authorization",
            "Bearer 55006dd7d868409c694628081e43f6ce5d1cee174943d8fcb03ca66507390427",
        )
        .header("cache-control", "no-cache")
        .header("dnt", "1")
        .header("origin", "https://magic.gg")
        .header("pragma", "no-cache")
        .header("priority", "u=1, i")
        .header("referer", "https://magic.gg/")
        .header("sec-ch-ua", r#""Not)A;Brand";v="8", "Chromium";v="138""#)
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", r#""macOS""#)
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "cross-site")
        .header(
            "x-contentful-user-agent",
            "sdk contentful.js/0.0.0-determined-by-semantic-release; platform browser; os macOS;",
        )
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(eyre!(
            "Failed to fetch deck lists: HTTP {}",
            response.status()
        ));
    }

    let response_text = response.text().await?;
    let contentful_response: ContentfulResponse = serde_json::from_str(&response_text)
        .map_err(|e| eyre!("Failed to parse response: {}", e))?;

    // Cache each item and generate IDs
    let cache_manager = crate::cache::CacheManager::new()?;
    let mut items_with_ids = Vec::new();

    for item in &contentful_response.items {
        let item_hash = crate::decks::generate_short_hash(&item);

        // Construct the link
        let link = if let Some(outbound) = &item.fields.outbound_link {
            outbound.clone()
        } else {
            format!("https://magic.gg/decklists/{}", item.fields.slug)
        };

        // Cache the item with the link included
        let mut item_json = serde_json::to_value(item)?;
        if let Some(obj) = item_json.as_object_mut() {
            obj.insert("link".to_string(), serde_json::Value::String(link));
        }
        cache_manager.set(&item_hash, item_json).await?;

        items_with_ids.push(ContentfulItemWithId {
            id: item_hash,
            item: item.clone(),
        });
    }

    // Output results
    match output.as_str() {
        "json" => output_ranked_json_with_ids(&contentful_response, &items_with_ids)?,
        "pretty" => output_ranked_pretty_with_ids(
            &items_with_ids,
            contentful_response.total,
            contentful_response.skip,
            format_filter,
            limit,
        )?,
        _ => output_ranked_pretty_with_ids(
            &items_with_ids,
            contentful_response.total,
            contentful_response.skip,
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
