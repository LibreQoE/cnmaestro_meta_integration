//! Quick read/write for ShapedDevices.csv

use anyhow::{bail, Result};
use csv::{ReaderBuilder, StringRecord};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{error, warn};

// circuit_id,circuit_name,device_id,device_name,parent_node,mac,ipv4,ipv6,download_min,upload_min,download_max,upload_max,comment
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct SerializableShapedDevice {
    pub circuit_id: String,
    pub circuit_name: String,
    pub device_id: String,
    pub device_name: String,
    pub parent_node: String,
    pub mac: String,
    pub ipv4: String,
    pub ipv6: String,
    pub download_min_mbps: u32,
    pub upload_min_mbps: u32,
    pub download_max_mbps: u32,
    pub upload_max_mbps: u32,
    pub comment: String,
}

impl SerializableShapedDevice {
    fn from_csv(record: &StringRecord) -> Result<Self> {
        Ok(Self {
            circuit_id: record[0].to_string(),
            circuit_name: record[1].to_string(),
            device_id: record[2].to_string(),
            device_name: record[3].to_string(),
            parent_node: record[4].to_string(),
            mac: record[5].to_string(),
            ipv4: record[6].to_string(),
            ipv6: record[7].to_string(),
            download_min_mbps: record[8].parse()?,
            upload_min_mbps: record[9].parse()?,
            download_max_mbps: record[10].parse()?,
            upload_max_mbps: record[11].parse()?,
            comment: record[12].to_string(),
        })
    }
}

pub fn read_shaped_devices(path: &str) -> Result<Vec<SerializableShapedDevice>> {
    // Check the path
    let final_path = Path::new(path);
    if !final_path.exists() {
        error!("Shaped Devices not found at {}", path);
        bail!("Shaped Devices not found at {}", path);
    }

    // Build the reader and deserialize
    let reader = ReaderBuilder::new()
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_path(final_path);
    if reader.is_err() {
        error!("Unable to read {}", path);
        bail!("Unable to read {}", path);
    }
    let mut result = Vec::new();
    let mut reader = reader.unwrap();
    for (line, row) in reader.records().enumerate() {
        if let Ok(row) = row {
            let record = SerializableShapedDevice::from_csv(&row);
            if let Ok(record) = record {
                result.push(record);
            } else {
                warn!("Error parsing line {line}");
                warn!("{record:?}");
            }
        } else {
            warn!("Error reading line {line}");
            warn!("{row:?}");
        }
    }

    Ok(result)
}
