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
    Empty {
        name: super::AcName,
        mode: Option<String>,
        #[serde(rename="readOnly")]
        readonly: Option<bool>,
        uid: Option<u32>,
        gid: Option<u32>,
    },
    #[serde(rename="host")]
    Host {
        name: super::AcName,
        source: PathBuf,
        #[serde(rename="readOnly")]
        readonly: Option<bool>,
        recursive: Option<bool>,
    },
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
