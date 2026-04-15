use crate::license::{License, is_license};
use anyhow::{Context, anyhow};
use serde::Deserialize;
use std::path::Path;
use url::Url;

pub type Remote = License<Url>;

pub fn package_remote_licenses(
    keywords: &[String],
    package: &str,
    repo_url: &str,
) -> anyhow::Result<impl Iterator<Item = Remote>> {
    Ok(ureq::Agent::new_with_defaults()
        .get(&api_url_from_repo_url(repo_url)?)
        .header("User-Agent", "root-lister/1.0")
        .header("Accept", "application/vnd.github+json")
        .call()?
        .into_body()
        .read_json::<Vec<GithubFileInfo>>()?
        .into_iter()
        .filter(|file| is_license(keywords, &file.name))
        .map(|file| Remote {
            package: package.to_string(),
            location: file.download_url.unwrap(),
            name: file.name,
        }))
}

pub fn download(url: &Url, output: &Path) -> anyhow::Result<()> {
    std::io::copy(
        &mut ureq::get(url.as_str()).call()?.into_body().into_reader(),
        &mut std::fs::File::create(output)?,
    )?;
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
