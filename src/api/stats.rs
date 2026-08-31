use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WikiStatistics {
    pub articles: u64,
    pub pages: u64,
    pub edits: u64,
    pub activeusers: u64,
}

impl Default for WikiStatistics {
    fn default() -> Self {
        Self {
            articles: 7_200_000,
            pages: 66_000_000,
            edits: 1_360_000_000,
            activeusers: 250_000,
        }
    }
}

#[derive(Debug, Deserialize)]
struct SiteInfoQuery {
    statistics: WikiStatistics,
}

#[derive(Debug, Deserialize)]
struct SiteInfoResponse {
    query: SiteInfoQuery,
}

pub fn format_metric(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.0}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

pub fn fetch_wiki_statistics(
    agent: &ureq::Agent,
    timeout_secs: u64,
) -> Result<WikiStatistics, String> {
    let url = "https://en.wikipedia.org/w/api.php?action=query&meta=siteinfo&siprop=statistics&format=json";
    let data: SiteInfoResponse =
        super::send_request_json(agent.get(url), timeout_secs).map_err(|e| e.to_string())?;
    Ok(data.query.statistics)
}
