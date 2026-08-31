use super::article::fetch_article_wikipedia;
use serde::Deserialize;

#[derive(Deserialize)]
struct WikiRandomItem {
    title: String,
}

#[derive(Deserialize)]
struct WikiRandomQuery {
    random: Vec<WikiRandomItem>,
}

#[derive(Deserialize)]
struct WikiRandomResponse {
    query: Option<WikiRandomQuery>,
}

pub fn fetch_random_article(
    agent: &ureq::Agent,
    timeout_secs: u64,
    offline_cache: bool,
    cache_lifetime: u64,
) -> Result<(String, String), super::ApiError> {
    let url = "https://en.wikipedia.org/w/api.php";
    let req = agent
        .get(url)
        .query("action", "query")
        .query("list", "random")
        .query("rnnamespace", "0")
        .query("rnlimit", "1")
        .query("format", "json");

    let rand_resp: WikiRandomResponse = super::send_request_json(req, timeout_secs)?;

    let title = rand_resp
        .query
        .and_then(|q| q.random.into_iter().next())
        .map(|r| r.title)
        .ok_or_else(|| super::ApiError::NotFound("no random article returned".to_string()))?;

    let content =
        fetch_article_wikipedia(agent, &title, timeout_secs, offline_cache, cache_lifetime)?;
    Ok((title, content))
}
