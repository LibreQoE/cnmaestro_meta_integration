use crate::api::auth::{authorize, Authorization};
use crate::api::paging::Wrapper;
use crate::api::{Device, DeviceStats};
use anyhow::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;

/// Debug version: retrieve the endpoint response as
/// plain text and print it.
#[allow(dead_code)]
async fn get_debug_txt(auth: &Authorization, base_url: &str, api_url: &str) -> Result<()> {
    let full_url = format!("{base_url}/api/v1/{}?offset=0", api_url);
    let res = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?
        .get(&full_url)
        .header("'Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", auth.access_token))
        .send()
        .await?
        .text()
        .await?;

    println!("{res}");
    Ok(())
}

async fn get_vec<T: DeserializeOwned + Clone>(
    auth: &Authorization,
    base_url: &str,
    api_url: &str,
) -> Result<Vec<T>, anyhow::Error> {
    let mut result = Vec::new();

    let mut offset = 0;
    loop {
        let full_url = format!("{base_url}/api/v1/{}?offset={offset}", api_url);
        let res = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?
            .get(&full_url)
            .header("'Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", auth.access_token))
            .send()
            .await
            .unwrap();

        let data_page = res.json::<Wrapper<T>>().await?;
        result.extend_from_slice(&data_page.data);
        if offset + data_page.paging.limit > data_page.paging.total {
            break;
        }
        offset += data_page.paging.limit;
    }

    Ok(result)
}

pub async fn get_devices(base_url: &str, username: &str, password: &str) -> Result<Vec<Device>> {
    let auth = authorize(base_url, username, password).await?;
    Ok(get_vec::<Device>(&auth, base_url, "devices").await?)
}

#[allow(dead_code)]
pub async fn debug_print_devices(base_url: &str, username: &str, password: &str) -> Result<()> {
    let auth = authorize(base_url, username, password).await?;
    get_debug_txt(&auth, base_url, "devices").await?;
    Ok(())
}

pub async fn get_device_stats(
    base_url: &str,
    username: &str,
    password: &str,
) -> Result<Vec<DeviceStats>> {
    let auth = authorize(base_url, username, password).await?;
    Ok(get_vec::<DeviceStats>(&auth, base_url, "devices/statistics").await?)
}

#[allow(dead_code)]
pub async fn debug_print_stats(base_url: &str, username: &str, password: &str) -> Result<()> {
    let auth = authorize(base_url, username, password).await?;
    get_debug_txt(&auth, base_url, "devices/statistics").await?;
    Ok(())
}
