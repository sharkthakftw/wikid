use serde::Deserialize;

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

pub fn check_latest_release(
    agent: &ureq::Agent,
    timeout_secs: u64,
) -> Result<String, super::ApiError> {
    let url = "https://api.github.com/repos/sharkthakftw/wikid/releases/latest";
    let release: GitHubRelease = super::send_request_json(agent.get(url), timeout_secs)?;
    Ok(release.tag_name)
}
