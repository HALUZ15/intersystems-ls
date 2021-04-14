use crate::{
  connection::ConnectionManager,
  feature::{FeatureProvider, FeatureRequest},
  protocol::*,
};
use async_trait::async_trait;
use log::{error};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

pub struct CompileProvider<C> {
  client: Arc<C>,
  connection_manager: Arc<ConnectionManager>,
  // handles_by_token: Mutex<HashMap<ProgressToken, AbortHandle>>,
  // current_docs: CHashMap<Uri, ()>,
}

impl<C> CompileProvider<C> {
  pub fn new(client: Arc<C>, connection_manager: Arc<ConnectionManager>) -> Self {
    Self {
      client,
      connection_manager,
      // handles_by_token: Mutex::new(HashMap::new()),
      // current_docs: CHashMap::new(),
    }
  }
  fn connection_manager(&self) -> &ConnectionManager {
    &self.connection_manager
    // .get()
    // .expect("initialize has not been called")
  }
}

#[async_trait]
impl<C> FeatureProvider for CompileProvider<C>
where
  C: LspClient + Send + Sync + 'static,
{
  type Params = CompileParams;
  type Output = CompileResult;

  async fn execute<'a>(&'a self, req: &'a FeatureRequest<CompileParams>) -> CompileResult {
    let doc = req.current();
    let path: PathBuf = doc.uri.to_file_path().unwrap();
    let filename = path.file_name().unwrap().to_string_lossy().to_owned();

    if let Some(mut conn) = self.connection_manager().connect() {
      let mut lines: Vec<String> = Vec::new();
      for line in doc.text.lines() {
        lines.push(String::from(line));
      }
      while lines[lines.len() - 1].len() == 0 {
        lines.pop();
      }
      // new_content.push(String::default());
    let mut loaded: String = String::default();
      let mut success = false;
      let output = conn.load(&filename, "ck", lines.clone(), &mut loaded, &mut success);
      self
        .client
        .log_message(LogMessageParams {
          typ: MessageType::Log,
          message: output,
        })
        .await;
      if success {
        let mut new_content: Vec<String> = conn.export_udl(loaded.as_str());
        while new_content[new_content.len() - 1].len() == 0 {
          new_content.pop();
        }
        // new_content.push(String::default());
        if new_content.len() > 0 && new_content != lines {
          let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
          file.write(new_content[0].as_bytes()).unwrap();
          new_content.remove(0);
          for line in new_content {
            file.write(b"\n").unwrap();
            file.write(line.as_bytes()).unwrap();
          }
          file.flush().unwrap();
        }
        CompileResult {
          status: CompileStatus::Success,
        }
      } else {
        CompileResult {
          status: CompileStatus::Error,
        }
      }
    } else {
      error!("Connection error");
      let status = CompileStatus::Error;
      CompileResult { status }
    }
  }
}
