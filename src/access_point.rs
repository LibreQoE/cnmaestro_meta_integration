use crate::api::{Device, DeviceStats};
use anyhow::bail;

#[derive(Debug)]
pub struct AccessPoint {
    pub name: String,
    pub mac: String,
    pub maestro_sm_count: i32,
}

impl AccessPoint {
    pub fn new(devices: &[Device], stats: &[DeviceStats], mac: &str) -> anyhow::Result<Self> {
        if let (Some(device), Some(stats)) = (
            devices.iter().find(|d| d.mac.as_ref().unwrap() == mac),
            (stats.iter().find(|d| d.mac.as_ref().unwrap() == mac)),
        ) {
            Ok(Self {
                name: device.name.clone().unwrap(),
                mac: device.mac.clone().unwrap(),
                maestro_sm_count: stats.connected_sms.unwrap(),
            })
        } else {
            bail!("No matching device + stats");
        }
    }
}
