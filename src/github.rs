//use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

/// Representation of a single commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommit {
    /// SHA.
    pub sha: String,
    /// Author of the commit.
    pub author: Option<GitHubCommitAuthor>,
    /// Details of the commit
    pub commit: Option<GitHubCommitDetails>,
}

/// Representation of subset of commit details
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommitDetails {
    /// Author of the commit
    pub author: GitHubCommitDetailsAuthor,
}

/// Representation of subset of commit author details
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommitDetailsAuthor {
    /// Date of the commit
    pub date: String,
}

/// Author of the commit.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubCommitAuthor {
    /// Username.
    pub login: Option<String>,
}

/// Label of the pull request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestLabel {
    /// Name of the label.
    pub name: String,
}
