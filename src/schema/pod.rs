//! Schema types related to AppC Pods.

use serde_json;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub name: super::AcName,
    pub image: Image,
    pub app: Option<super::image::AppImage>,
    #[serde(rename = "readOnlyRootFs")]
    pub readonly_rootfs: Option<bool>,
    pub mounts: Option<Vec<AppMount>>,
    pub annotations: Option<Vec<Annotation>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppMount {
    pub volume: super::AcName,
    pub path: PathBuf,
    #[serde(rename = "appVolume")]
    pub app_volume: Option<Volume>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: super::ImageID,
    pub name: Option<super::AcIdentifier>,
    pub labels: Option<Vec<super::NameValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="kind")]
pub enum Volume {
    #[serde(rename="empty")]
    Empty(VolumeEmpty),
    #[serde(rename="host")]
    Host(VolumeHost),
}

/// A volume entry of kind `empty`.
#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeEmpty {
    pub name: super::AcName,
    pub mode: Option<String>,
    #[serde(rename="readOnly")]
    pub readonly: Option<bool>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
}

/// A volume entry of kind `host`.
#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeHost {
    pub name: super::AcName,
    pub source: PathBuf,
    #[serde(rename="readOnly")]
    pub readonly: Option<bool>,
    pub recursive: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Isolator {
    pub name: super::AcIdentifier,
    pub value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotation {
    pub name: super::AcName,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    pub name: super::AcName,
    #[serde(rename="hostPort")]
    pub host_port: u32,
    #[serde(rename="hostIP")]
    pub host_ip: Option<String>,
    #[serde(rename="podPort")]
    pub pod_port: Option<String>,
}
