//! AppC spec.

pub mod image;
pub mod pod;

/// AppC spec revision implemented by this library.
pub static APPC_REVISION: &'static str = "0.8.10";

pub type AcName = String;
pub type AcIdentifier = String;
pub type AcVersion = String; // semver::Version
pub type ImageID = String; // sha2::Sha512

/// App Container manifest kind.
#[derive(Serialize, Deserialize, Debug)]
pub enum AcKind {
    ImageManifest,
    PodManifest,
}

/// A keyed tuple (name + value).
#[derive(Serialize, Deserialize, Debug)]
pub struct NameValue {
    pub name: String,
    pub value: String,
}

/// App Container Pod Manifest.
#[derive(Serialize, Deserialize, Debug)]
pub struct PodManifest {
    #[serde(rename = "acVersion")]
    pub ac_version: AcVersion,
    #[serde(rename = "acKind")]
    pub ac_kind: AcKind,
    pub apps: Vec<pod::App>,
    pub volumes: Option<Vec<pod::Volume>>,
    pub isolators: Option<Vec<pod::Isolator>>,
    pub annotations: Option<Vec<pod::Annotation>>,
    pub ports: Option<Vec<pod::Port>>,
    #[serde(rename = "user_annotations")]
    pub user_annotations: Option<Vec<(String, String)>>,
    #[serde(rename = "user_labels")]
    pub user_labels: Option<Vec<(String, String)>>,
}

/// App Container Image (ACI) Manifest.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageManifest {
    #[serde(rename = "acVersion")]
    pub ac_version: AcVersion,
    #[serde(rename = "acKind")]
    pub ac_kind: AcKind,
    pub name: String,
    pub labels: Option<Vec<NameValue>>,
    pub app: Option<image::AppImage>,
    pub dependencies: Option<Vec<image::Dependency>>,
    #[serde(rename = "pathWhitelist")]
    pub path_whitelist: Option<Vec<String>>,
    pub annotations: Option<Vec<NameValue>>,
}
