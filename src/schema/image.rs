//! Schema types related to AppC Images.

use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppImage {
    pub exec: Option<Vec<String>>,
    pub user: String,
    pub group: String,
    #[serde(rename = "workingDirectory")]
    pub working_dir: Option<PathBuf>,
    pub environment: Option<Vec<super::NameValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    #[serde(rename = "imageName")]
    pub image_name: super::AcIdentifier,
    #[serde(rename = "imageID")]
    pub image_id: super::ImageID,
    pub labels: Option<Vec<super::NameValue>>,
    pub size: Option<u64>,
}
