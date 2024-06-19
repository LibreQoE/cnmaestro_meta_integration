use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Device {
    pub country: Option<String>,
    pub name: Option<String>,
    pub ip: Option<String>,
    pub network: Option<String>,
    pub product: Option<String>,
    pub msn: Option<String>,
    pub software_version: Option<String>,
    pub cbrs_state: Option<String>,
    pub cbrs_status: Option<String>,
    pub last_reboot_reason: Option<String>,
    pub hardware_version: Option<String>,
    pub registration_date: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub site_type: Option<String>,
    pub tower: Option<String>,
    pub mac: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct DeviceStats {
    pub name: Option<String>,
    pub mac: Option<String>,
    pub status: Option<String>,
    pub connected_sms: Option<i32>,
    pub ap_mac: Option<String>,
}
