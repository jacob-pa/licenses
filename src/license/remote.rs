use crate::license::is_license;
use anyhow::{Context, anyhow};
use serde::Deserialize;
use std::path::Path;
use url::Url;

pub struct RemoteLicense {
    pub name: String,
    pub location: Url,
}

pub fn package_remote_licenses(
    keywords: &[String],
    repo_url: &str,
) -> anyhow::Result<impl Iterator<Item = RemoteLicense>> {
    Ok(ureq::Agent::new_with_defaults()
        .get(&api_url_from_repo_url(repo_url)?)
        .header("User-Agent", "root-lister/1.0")
        .header("Accept", "application/vnd.github+json")
        .call()?
        .into_body()
        .read_json::<Vec<GithubFileInfo>>()?
        .into_iter()
        .filter(|file| is_license(keywords, &file.name))
        .map(|file| RemoteLicense {
            location: file.download_url.unwrap(),
            name: file.name,
        }))
}

pub fn download(license: &RemoteLicense, output: &Path) -> anyhow::Result<()> {
    let mut content = ureq::get(license.location.as_str())
        .call()?
        .into_body()
        .into_reader();
    std::io::copy(&mut content, &mut std::fs::File::create(output)?)?;
    Ok(())
}

fn api_url_from_repo_url(repo_url: &str) -> anyhow::Result<String> {
    let url = Url::parse(repo_url).context("invalid URL")?;
    if url.host_str() != Some("github.com") {
        return Err(anyhow!("not a github.com URL: {repo_url}"));
    }
    let mut segments = url
        .path_segments()
        .ok_or_else(|| anyhow!("no path segments in URL"))?;
    let owner = segments.next().ok_or_else(|| anyhow!("missing owner"))?;
    let name = segments
        .next()
        .ok_or_else(|| anyhow!("missing repo"))?
        .trim_end_matches(".git");
    Ok(format!(
        "https://api.github.com/repos/{owner}/{name}/contents/"
    ))
}

#[derive(Deserialize)]
struct GithubFileInfo {
    name: String,
    download_url: Option<Url>,
}
