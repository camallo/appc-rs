//! AppC spec.

mod image;

pub mod pod;

/// AppC spec revision implemented by this library.
pub static APPC_REVISION: &'static str = "0.8.6";

pub type AcName = String;
pub type AcIdentifier = String;
pub type AcVersion = String; // semver::Version

#[derive(Serialize, Deserialize, Debug)]
pub enum AcKind {
    ImageManifest,
    PodManifest,
}

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
