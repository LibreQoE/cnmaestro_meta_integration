use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Authorization {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
}

pub async fn authorize(
    base_url: &str,
    username: &str,
    password: &str,
) -> Result<Authorization, anyhow::Error> {
    let request =
        format!("grant_type=client_credentials&client_id={username}&client_secret={password}");
    let full_url = format!("{base_url}/api/v1/access/token");

    let res = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?
        .post(&full_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(request)
        .send()
        .await
        .unwrap();

    let result = res.json().await?;
    Ok(result)
}
