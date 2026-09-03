use crate::feed::algorithm::FeedItem;
use serde::Deserialize;

#[derive(Deserialize)]
struct WikiCategoryItem {
    title: String,
}

#[derive(Deserialize)]
struct WikiPageProp {
    title: Option<String>,
    description: Option<String>,
    extract: Option<String>,
    categories: Option<Vec<WikiCategoryItem>>,
}

#[derive(Deserialize)]
struct WikiFeedQuery {
    pages: Option<std::collections::HashMap<String, WikiPageProp>>,
}

#[derive(Deserialize)]
struct WikiFeedResponse {
    query: Option<WikiFeedQuery>,
}

fn parse_feed_items(query: WikiFeedQuery) -> Vec<FeedItem> {
    let mut items = Vec::new();
    if let Some(pages) = query.pages {
        for (_, page) in pages {
            if let Some(title) = page.title {
                let short_description = page.description.filter(|d| !d.trim().is_empty());
                let snippet = page.extract.unwrap_or_default().trim().to_string();
                let mut categories: Vec<String> = page
                    .categories
                    .unwrap_or_default()
                    .into_iter()
                    .map(|c| {
                        if let Some(stripped) = c.title.strip_prefix("Category:") {
                            stripped.to_string()
                        } else {
                            c.title
                        }
                    })
                    .filter(|cat| {
                        let lower = cat.to_lowercase();
                        !lower.starts_with("all ")
                            && !lower.starts_with("articles ")
                            && !lower.starts_with("cs1 ")
                            && !lower.contains("stubs")
                            && !lower.contains("tracking")
                    })
                    .collect();

                if categories.is_empty() {
                    categories = title
                        .split(|c: char| !c.is_alphanumeric())
                        .filter(|w| w.len() > 3)
                        .map(|w| w.to_lowercase())
                        .take(3)
                        .collect();
                }

                items.push(FeedItem {
                    title,
                    short_description,
                    snippet,
                    categories,
                    is_liked: false,
                });
            }
        }
    }
    items
}

fn query_feed_items(req: ureq::Request, timeout_secs: u64) -> Result<Vec<FeedItem>, String> {
    let req = req
        .query("action", "query")
        .query("prop", "description|extracts|categories")
        .query("exintro", "1")
        .query("explaintext", "1")
        .query("clshow", "!hidden")
        .query("cllimit", "15")
        .query("format", "json");
    let feed_resp: WikiFeedResponse =
        super::send_request_json(req, timeout_secs).map_err(|e| e.to_string())?;
    Ok(feed_resp.query.map(parse_feed_items).unwrap_or_default())
}

fn fetch_category_items(
    agent: &ureq::Agent,
    category: &str,
    timeout_secs: u64,
) -> Result<Vec<FeedItem>, String> {
    let url = "https://en.wikipedia.org/w/api.php";
    let category_title = format!("Category:{}", category);
    let req = agent
        .get(url)
        .query("generator", "categorymembers")
        .query("gcmtitle", &category_title)
        .query("gcmtype", "page")
        .query("gcmlimit", "2");
    query_feed_items(req, timeout_secs)
}

fn fetch_random_items(agent: &ureq::Agent, timeout_secs: u64) -> Result<Vec<FeedItem>, String> {
    let url = "https://en.wikipedia.org/w/api.php";
    let req = agent
        .get(url)
        .query("generator", "random")
        .query("grnnamespace", "0")
        .query("grnlimit", "3");
    query_feed_items(req, timeout_secs)
}

pub fn fetch_feed_batch(agent: &ureq::Agent, timeout_secs: u64) -> Result<Vec<FeedItem>, String> {
    let profile = crate::feed::profile::FeedProfile::load();
    let active_subcats = profile.get_active_subcategories();

    let mut chosen_cats = Vec::new();
    if !active_subcats.is_empty() {
        let mut available = active_subcats.clone();
        crate::feed::algorithm::shuffle(&mut available);
        chosen_cats = available.into_iter().take(3).collect();
    }

    let mut items = Vec::new();
    let mut handles = Vec::new();

    for cat in chosen_cats {
        let agent = agent.clone();
        handles.push(std::thread::spawn(move || {
            fetch_category_items(&agent, &cat, timeout_secs)
        }));
    }

    let agent_rand = agent.clone();
    handles.push(std::thread::spawn(move || {
        fetch_random_items(&agent_rand, timeout_secs)
    }));

    for handle in handles {
        if let Ok(Ok(batch)) = handle.join() {
            items.extend(batch);
        }
    }

    crate::feed::algorithm::shuffle(&mut items);
    Ok(items)
}
