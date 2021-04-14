use crate::{
  protocol::*,
  workspace::{Document, Snapshot},
};
use async_trait::async_trait;
use std::{
  path::{Path, PathBuf},
  sync::Arc,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DocumentView {
  pub snapshot: Arc<Snapshot>,
  pub current: Arc<Document>,
  // pub related: Vec<Arc<Document>>,
}

impl DocumentView {
  pub fn analyze(
    snapshot: Arc<Snapshot>,
    current: Arc<Document>,
    _options: &Options,
    _current_dir: &Path,
  ) -> Self {
    // let related = snapshot.relations(&current.uri, options, current_dir);
    Self { snapshot, current }
  }
}

#[derive(Clone)]
pub struct FeatureRequest<P> {
  pub params: P,
  pub view: DocumentView,
  pub client_capabilities: Arc<ClientCapabilities>,
  pub options: Options,
  pub current_dir: Arc<PathBuf>,
}

impl<P> FeatureRequest<P> {
  pub fn snapshot(&self) -> &Snapshot {
    &self.view.snapshot
  }

  pub fn current(&self) -> &Document {
    &self.view.current
  }

  // pub fn related(&self) -> &[Arc<Document>] {
  //   &self.view.related
  // }
}

#[async_trait]
pub trait FeatureProvider {
  type Params;
  type Output;

  async fn execute<'a>(&'a self, req: &'a FeatureRequest<Self::Params>) -> Self::Output;
}

type ListProvider<P, O> = Box<dyn FeatureProvider<Params = P, Output = Vec<O>> + Send + Sync>;

#[derive(Default)]
pub struct ConcatProvider<P, O> {
  providers: Vec<ListProvider<P, O>>,
}

impl<P, O> ConcatProvider<P, O> {
  pub fn new(providers: Vec<ListProvider<P, O>>) -> Self {
    Self { providers }
  }
}

#[async_trait]
impl<P, O> FeatureProvider for ConcatProvider<P, O>
where
  P: Send + Sync,
  O: Send + Sync,
{
  type Params = P;
  type Output = Vec<O>;

  async fn execute<'a>(&'a self, req: &'a FeatureRequest<P>) -> Vec<O> {
    let mut items = Vec::new();
    for provider in &self.providers {
      items.append(&mut provider.execute(req).await);
    }
    items
  }
}

type OptionProvider<P, O> = Box<dyn FeatureProvider<Params = P, Output = Option<O>> + Send + Sync>;

#[derive(Default)]
pub struct ChoiceProvider<P, O> {
  providers: Vec<OptionProvider<P, O>>,
}

impl<P, O> ChoiceProvider<P, O> {
  pub fn new(providers: Vec<OptionProvider<P, O>>) -> Self {
    Self { providers }
  }
}

#[async_trait]
impl<P, O> FeatureProvider for ChoiceProvider<P, O>
where
  P: Send + Sync,
  O: Send + Sync,
{
  type Params = P;
  type Output = Option<O>;

  async fn execute<'a>(&'a self, req: &'a FeatureRequest<P>) -> Option<O> {
    for provider in &self.providers {
      let item = provider.execute(req).await;
      if item.is_some() {
        return item;
      }
    }
    None
  }
}
