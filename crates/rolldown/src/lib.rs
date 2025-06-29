mod asset;
mod ast_scanner;
mod bundler;
mod bundler_builder;
mod chunk_graph;
mod css;
mod ecmascript;
mod hmr;
mod module_finalizers;
mod module_loader;
mod stages;
mod type_alias;
mod types;
mod utils;
mod watch;
mod watcher;
use std::sync::Arc;

use rolldown_fs::OsFileSystem;
use rolldown_resolver::Resolver;

pub(crate) type SharedResolver = Arc<Resolver<OsFileSystem>>;
pub(crate) type SharedOptions = SharedNormalizedBundlerOptions;

pub use crate::{
  bundler::Bundler,
  bundler_builder::BundlerBuilder,
  types::bundle_output::BundleOutput,
  watch::event::{BundleEvent, WatcherEvent},
  watcher::Watcher,
};

pub use rolldown_common::bundler_options::*;

pub use rolldown_resolver::ResolveOptions;

pub use rolldown_plugin as plugin;

pub use oxc as rolldown_oxc;
