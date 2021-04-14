cfg_if::cfg_if! {
    if #[cfg(feature = "server")] {
        mod client;
        mod codec;

        pub use self::{
            client::{InterSystemsLspClient, LspClient},
            codec::LspCodec,
        };
    }
}

mod capabilities;
mod options;
mod uri;

pub use self::{
  capabilities::ClientCapabilitiesExt,
  options::*,
  uri::{AsUri, Uri},
};

pub use lsp_types::*;

use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Production {
  pub id: String,
  pub status: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ProductionService {
  pub id: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ProductionOperation {
  pub id: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ProductionProcess {
  pub id: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionsResult {
  pub list: Vec<Production>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionServicesResult {
  pub list: Vec<ProductionService>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionProcessesResult {
  pub list: Vec<ProductionProcess>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionOperationsResult {
  pub list: Vec<ProductionOperation>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Global {
  pub name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalsResult {
  pub list: Vec<Global>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Job {
  pub id: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobsResult {
  pub list: Vec<Job>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterSystemsConnectedParams {
  pub version: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileParams {
    pub text_document: TextDocumentIdentifier,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CompileStatus {
    Success = 0,
    Error = 1,
    Failure = 2,
    Cancelled = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileResult {
    pub status: CompileStatus,
}
