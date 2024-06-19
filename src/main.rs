//! This is a special integration. cnMaestro doesn't provide much in the way of billing
//! information - but provides a great picture of the current state of your network.
//!
//! Based on BracketQos ( https://github.com/thebracket/bracket-qos/blob/main/cnmaestro_support/src/lib.rs )
//! with permission.
//!
//! See also: https://docs.cloud.cambiumnetworks.com/api/2.4.0/index.html
//!
//! To actually use this:
//! * You need a cnMaestro that has the v1 API.
//! * Set CNMAESTRO_URL, CNMAESTRO_USERNAME and CNMAESTRO_SECRET environment variables (either directly or via a `.env` file).

use anyhow::Result;
use tracing::{info, warn};

use crate::access_point::AccessPoint;
use crate::api::{get_device_stats, get_devices, Device};
use crate::config::CnMaestroConfig;
use crate::shaped_devices::{read_shaped_devices, write_shaped_devices};
use crate::sm::SM;

mod access_point;
mod api;
mod config;
mod shaped_devices;
mod sm;

/// Sets up tracing for pretty output
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .compact()
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    let config = CnMaestroConfig::from_env()?;

    info!("Loading ShapedDevices from {}", &config.shaped_devices_path);
    let mut shaped_devices = read_shaped_devices(&config.shaped_devices_path)?;
    info!("Loaded {} records", shaped_devices.len());

    info!("Connecting to cnMaestro at {}", config.url);

    // Retrieve devices and only keep the ones with a MAC address
    let devices = get_devices(&config.url, &config.username, &config.secret).await?;
    info!("Found {} devices on cnMaestro", devices.len());

    let device_stats = get_device_stats(&config.url, &config.username, &config.secret).await?;
    info!("Found {} device statistics records", device_stats.len());

    // Filter out devices that don't have a Name, Mac, IP or entry in the stats
    let devices: Vec<Device> = devices
        .into_iter()
        .filter(|d| d.mac.is_some() && d.ip.is_some() && d.name.is_some())
        .filter(|d| device_stats.iter().any(|ds| ds.mac == d.mac))
        .collect();
    info!("Retained {} devices with necessary data.", devices.len());

    // Build the Access Point List
    let aps: Vec<AccessPoint> = device_stats
        .iter()
        .filter(|ds| ds.connected_sms.unwrap_or(0) > 0)
        .map(|ds| AccessPoint::new(&devices, &device_stats, ds.mac.as_ref().unwrap()))
        .flatten()
        .collect();
    info!("Found {} cnMaestro Access Points (AP)", aps.len());

    // Build the SM List
    let sms: Vec<SM> = devices
        .iter()
        .filter(|d| !aps.iter().any(|ap| ap.mac == d.mac.clone().unwrap()))
        .map(|d| SM::new(&devices, &device_stats, &aps, d))
        .flatten()
        .collect();
    info!("Found {} Subscriber Modules (SM)", sms.len());

    // Find matches with Shaped Devices
    let matched_macs: Vec<_> = sms
        .iter()
        .filter(|sm| {
            shaped_devices
                .iter()
                .any(|sd| sd.mac.to_lowercase() == sm.mac.to_lowercase())
        })
        .map(|sm| {
            (
                sm,
                shaped_devices
                    .iter()
                    .find(|sd| sd.mac.to_lowercase() == sm.mac.to_lowercase())
                    .unwrap(),
            )
        })
        .collect();
    info!(
        "Found {} SMs that have entries in Shaped Devices",
        matched_macs.len()
    );
    let unmatched_macs: Vec<_> = sms
        .iter()
        .filter(|sm| {
            !shaped_devices
                .iter()
                .any(|sd| sd.mac.to_lowercase() == sm.mac.to_lowercase())
        })
        .collect();
    warn!(
        "Found {} SMs that DO NOT have entries in Shaped Devices",
        unmatched_macs.len()
    );

    // Find IP mismatches
    let ip_mismatches: Vec<_> = matched_macs.iter()
        .filter(|(sm,shaped)| {
            let shaped_ipv4 = shaped.ipv4.replace("/32", "");
            sm.ip != shaped_ipv4
        })
        .map(|(sm, shaped)| {
            (sm.mac.clone(), sm.ip.clone())
        })
        .collect();

    info!("Found {} SMs with IPs different from the entry in ShapedDevices", ip_mismatches.len());

    // Apply the updates to Shaped Devices
    for (mac, ip) in ip_mismatches.iter() {
        if let Some(shaped) = shaped_devices.iter_mut().find(|sd| sd.mac.to_lowercase() == mac.to_lowercase()) {
            shaped.ipv4 = format!("{ip}/32");
        }
    }

    // Write Shaped Devices
    write_shaped_devices(&shaped_devices, &config.shaped_devices_path)?;

    Ok(())
}
