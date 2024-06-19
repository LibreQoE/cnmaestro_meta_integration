use crate::access_point::AccessPoint;
use crate::api::{Device, DeviceStats};
use anyhow::bail;

#[derive(Debug)]
pub struct SM {
    pub name: String,
    pub mac: String,
    pub ip: String,
    pub ap_mac: String,
    pub ap_name: String,
}

impl SM {
    pub fn new(
        devices: &[Device],
        stats: &[DeviceStats],
        aps: &[AccessPoint],
        device: &Device,
    ) -> anyhow::Result<Self> {
        let mac = device.mac.clone().unwrap();
        if let (Some(device), Some(stats)) = (
            devices.iter().find(|d| d.mac.clone().unwrap() == mac),
            (stats.iter().find(|d| d.mac.clone().unwrap() == mac)),
        ) {
            let ap_mac = stats.ap_mac.clone().unwrap_or("".to_string());
            let ap_name = if let Some(ap) = aps.iter().find(|ap| ap.mac == ap_mac) {
                ap.name.clone()
            } else {
                "".to_string()
            };
            Ok(Self {
                name: device.name.clone().unwrap(),
                mac,
                ip: device.ip.clone().unwrap(),
                ap_mac,
                ap_name,
            })
        } else {
            bail!("No matching device + stats");
        }
    }
}
