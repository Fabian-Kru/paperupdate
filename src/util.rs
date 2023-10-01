use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PaperVersion {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub builds: Vec<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaperBuildVersion {
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "project_name")]
    pub project_name: String,
    pub version: String,
    pub build: i64,
    pub time: String,
    pub channel: String,
    pub promoted: bool,
    pub changes: Vec<Change>,
    pub downloads: Downloads,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub commit: String,
    pub summary: String,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads {
    pub application: Application,
    #[serde(rename = "mojang-mappings")]
    pub mojang_mappings: MojangMappings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub name: String,
    pub sha256: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MojangMappings {
    pub name: String,
    pub sha256: String,
}
