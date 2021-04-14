use crate::protocol::{Options, TextDocumentItem, Uri};
use futures::lock::Mutex;
use log::debug;
use std::{
  hash::{Hash, Hasher},
  path::{Path, PathBuf},
  sync::Arc,
  time::SystemTime,
};

pub struct Workspace {
  current_dir: Arc<PathBuf>,
  snapshot: Mutex<Arc<Snapshot>>,
}

impl Workspace {
  pub fn new(current_dir: Arc<PathBuf>) -> Self {
    Self {
      current_dir,
      snapshot: Mutex::default(),
    }
  }

  pub async fn get(&self) -> Arc<Snapshot> {
    let snapshot = self.snapshot.lock().await;
    Arc::clone(&snapshot)
  }

  pub async fn add(&self, document: TextDocumentItem, options: &Options) {
    debug!("Adding document: {}", document.uri);
    let mut snapshot = self.snapshot.lock().await;
    *snapshot = self
      .add_or_update(&snapshot, document.uri.into(), document.text, options)
      .await;
  }

  pub async fn update(&self, uri: Uri, text: String, options: &Options) {
    let mut snapshot = self.snapshot.lock().await;
    *snapshot = self.add_or_update(&snapshot, uri, text, options).await;
  }

  async fn add_or_update(
    &self,
    snapshot: &Snapshot,
    uri: Uri,
    text: String,
    options: &Options,
  ) -> Arc<Snapshot> {
    let document = Document::open(DocumentParams {
      uri,
      text,
      options,
      current_dir: &self.current_dir,
    });

    let mut documents: Vec<Arc<Document>> = snapshot
      .0
      .iter()
      .filter(|x| x.uri != document.uri)
      .cloned()
      .collect();

    documents.push(Arc::new(document));
    Arc::new(Snapshot(documents))
  }

  pub async fn reparse(&self, _options: &Options) {}
}

#[derive(Debug, Clone)]
pub struct Document {
  pub uri: Uri,
  pub text: String,
  pub content: DocumentContent,
  pub modified: SystemTime,
}

impl Document {
  pub fn is_file(&self) -> bool {
    self.uri.scheme() == "file"
  }

  pub fn open(params: DocumentParams) -> Self {
    let DocumentParams {
      uri,
      text,
      ..
    } = params;

    let content = DocumentContent::Objectscript;

    Self {
      uri,
      text,
      content,
      modified: SystemTime::now(),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DocumentParams<'a> {
  pub uri: Uri,
  pub text: String,
  pub options: &'a Options,
  pub current_dir: &'a Path,
}

#[derive(Debug, Clone)]
pub enum DocumentContent {
  Objectscript,
}

impl PartialEq for Document {
  fn eq(&self, other: &Self) -> bool {
    self.uri == other.uri
  }
}

impl Eq for Document {}

impl Hash for Document {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.uri.hash(state);
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Snapshot(pub Vec<Arc<Document>>);

impl Snapshot {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  pub fn push(&mut self, doc: Document) {
    self.0.push(Arc::new(doc));
  }

  pub fn find(&self, uri: &Uri) -> Option<Arc<Document>> {
    self.0.iter().find(|doc| doc.uri == *uri).map(Arc::clone)
  }
}
