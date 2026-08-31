use serde::Deserialize;

#[derive(Deserialize)]
struct CategoryMember {
    title: String,
    ns: i32,
}

#[derive(Deserialize)]
struct CategoryMembersQuery {
    categorymembers: Option<Vec<CategoryMember>>,
}

#[derive(Deserialize)]
struct CategoryMembersResponse {
    query: Option<CategoryMembersQuery>,
}

pub fn fetch_category_members(
    agent: &ureq::Agent,
    category: &str,
    limit: usize,
    timeout_secs: u64,
) -> Result<Vec<String>, super::ApiError> {
    let url = "https://en.wikipedia.org/w/api.php";
    let cmtitle = if category.starts_with("Category:") {
        category.to_string()
    } else {
        format!("Category:{}", category)
    };
    let limit_str = limit.clamp(1, 100).to_string();

    let res = agent
        .get(url)
        .timeout(std::time::Duration::from_secs(timeout_secs.max(1)))
        .query("action", "query")
        .query("list", "categorymembers")
        .query("cmtitle", &cmtitle)
        .query("cmlimit", &limit_str)
        .query("cmnamespace", "0")
        .query("format", "json")
        .call()
        .map_err(|e| super::ApiError::Network(e.to_string()))?;

    let resp: CategoryMembersResponse = res
        .into_json()
        .map_err(|e| super::ApiError::Parse(e.to_string()))?;

    let articles = resp
        .query
        .and_then(|q| q.categorymembers)
        .unwrap_or_default()
        .into_iter()
        .filter(|m| m.ns == 0)
        .map(|m| m.title)
        .collect();

    Ok(articles)
}
