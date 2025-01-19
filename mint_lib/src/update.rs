use crate::error::GenericError;
use crate::error::ResultExt;

pub const GITHUB_RELEASE_URL: &str = "https://api.github.com/repos/iriscats/mint/releases/latest";
pub const GITHUB_REQ_USER_AGENT: &str = "iriscats/mint";

#[derive(Debug, serde::Deserialize)]
pub struct GitHubRelease {
    pub html_url: String,
    pub tag_name: String,
    pub body: String,
}

pub async fn get_latest_release() -> Result<GitHubRelease, GenericError> {
    reqwest::Client::builder()
        .user_agent(GITHUB_REQ_USER_AGENT)
        .build()
        .generic("无法构建reqwest客户端".to_string())?
        .get(GITHUB_RELEASE_URL)
        .send()
        .await
        .generic("检查自动更新请求失败".to_string())?
        .json::<GitHubRelease>()
        .await
        .generic("检查自动更新无响应".to_string())
}
