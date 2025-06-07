use std::collections::HashMap;

use anyhow::{Context, Result};
use futures::future::join_all;
use octocrab::Octocrab;
use semver::Version;

pub async fn releases(repo_url: &str) -> Result<Vec<String>, anyhow::Error> {
    let octocrab = match std::env::var("GITHUB_TOKEN") {
        Ok(token) => Octocrab::builder().personal_token(token).build()?,
        Err(_) => Octocrab::builder().build()?,
    };

    // Parse repo_url to extract owner and repo
    let (owner, repo) = if let Some(stripped) = repo_url.strip_prefix("https://github.com/") {
        let parts: Vec<&str> = stripped.trim_end_matches(".git").split('/').collect();
        (parts[0], parts[1])
    } else if repo_url.contains('/') {
        let parts: Vec<&str> = repo_url.trim_end_matches(".git").split('/').collect();
        (parts[0], parts[1])
    } else {
        return Err(anyhow::anyhow!("Invalid repo url: {repo_url}"));
    };

    let mut releases = Vec::new();
    let mut page = octocrab
        .repos(owner, repo)
        .releases()
        .list()
        .per_page(100)
        .send()
        .await
        .context("Failed to fetch releases")?;

    loop {
        releases.extend(page.items.iter().map(|release| {
            release
                .name
                .clone()
                .unwrap_or_else(|| release.tag_name.clone())
        }));
        if let Some(next_page) = octocrab
            .get_page::<octocrab::models::repos::Release>(&page.next)
            .await?
        {
            page = next_page;
        } else {
            break;
        }
    }

    releases.reverse(); // To match your original behavior
    Ok(releases)
}

pub async fn versions_with_prefixes(
    repo_url: &str,
    additional_filters: &[&str],
) -> Result<Vec<String>, anyhow::Error> {
    releases(repo_url)
        .await
        .map(|releases| {
            let mut versions = releases
                .iter()
                .map(|s| {
                    let mut lowercase = s.trim().to_lowercase();
                    for filter in additional_filters {
                        lowercase = lowercase.trim_start_matches(filter).to_string();
                    }
                    lowercase.trim_start_matches('v').trim().to_string()
                })
                .into_iter()
                .filter_map(|s| Version::parse(&s).ok())
                .collect::<Vec<Version>>();
            versions.sort_by(|a, b| b.cmp(a));
            versions
                .iter()
                .filter(|v| v.pre.is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .map_err(Into::into)
}

pub async fn versions(repo_url: &str) -> Result<Vec<String>, anyhow::Error> {
    versions_with_prefixes(repo_url, &[]).await
}

pub async fn versions_with(
    version_fns: HashMap<
        &str,
        fn() -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<Vec<String>, anyhow::Error>> + Send>,
        >,
    >,
) -> HashMap<String, Vec<String>> {
    // Run all async functions in parallel
    let futures = version_fns.iter().map(|(name, func)| {
        let name = name.to_string();
        async move {
            match func().await {
                Ok(versions) => vec![(name.clone(), versions)],
                Err(_) => vec![],
            }
        }
    });

    let results: Vec<Vec<(String, Vec<String>)>> = join_all(futures).await;
    results.into_iter().flatten().collect()
}

pub async fn latest_versions_with(
    version_fns: HashMap<
        &str,
        fn() -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<String, anyhow::Error>> + Send>,
        >,
    >,
) -> HashMap<String, String> {
    // Run all async functions in parallel
    let futures = version_fns.iter().map(|(name, func)| {
        let name = name.to_string();
        async move {
            match func().await {
                Ok(version) => vec![(name.clone(), version)],
                Err(_) => vec![],
            }
        }
    });

    let results: Vec<Vec<(String, String)>> = join_all(futures).await;
    results.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "skip_online_tests"))]
    #[tokio::test]
    async fn test_releases() {
        let tags = releases("https://github.com/kubernetes-sigs/descheduler.git")
            .await
            .unwrap();

        assert_eq!(
            tags.is_empty(),
            false,
            "descheduler releases should not be empty"
        );
    }
}
