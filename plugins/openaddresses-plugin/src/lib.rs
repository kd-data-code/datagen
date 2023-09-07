mod backends;
mod objects;

use crate::backends::backend::{Backend, BackendConstructor};
use crate::backends::memory_backend::MemoryBackend;
#[cfg(feature = "sqlite")]
use crate::backends::sqlite_backend::SQLiteBackend;
use crate::objects::args::{BackendType, IntoGenerated, PluginArgs, StringOrVec};
use crate::objects::call_args::CallArgs;
use datagen_rs::declare_plugin;
use datagen_rs::generate::current_schema::CurrentSchema;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::plugins::plugin::{Plugin, PluginConstructor};
use datagen_rs::util::types::Result;
#[cfg(feature = "log")]
use log::LevelFilter;
#[cfg(feature = "log")]
use log4rs::append::console::ConsoleAppender;
#[cfg(feature = "log")]
use log4rs::config::{Appender, Root};
#[cfg(feature = "log")]
use log4rs::Config;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct OpenAddressesPlugin {
    backend: Mutex<Box<dyn Backend>>,
}

impl Plugin for OpenAddressesPlugin {
    fn name(&self) -> &'static str {
        "openaddresses"
    }

    fn generate(&self, schema: Arc<CurrentSchema>, args: Value) -> Result<Arc<GeneratedSchema>> {
        let args: CallArgs = serde_json::from_value(args)?;
        let feature = self.backend.lock().unwrap().get_random_feature()?;

        args.into_generated(&schema, &feature)
    }
}

impl PluginConstructor for OpenAddressesPlugin {
    fn new(args: Box<Value>) -> Result<Self> {
        let args: PluginArgs = serde_json::from_value(*args)?;
        let paths = match args.files.clone() {
            StringOrVec::Single(path) => vec![path],
            StringOrVec::Multiple(paths) => paths,
        };

        #[cfg(feature = "log")]
        log4rs::init_config(
            Config::builder()
                .appender(
                    Appender::builder()
                        .build("stdout", Box::new(ConsoleAppender::builder().build())),
                )
                .build(Root::builder().appender("stdout").build(LevelFilter::Debug))?,
        )?;

        #[cfg(feature = "log")]
        log::debug!("Initializing plugin 'openaddress'");

        let backend: Box<dyn Backend> = match &args.backend.clone().unwrap_or_default() {
            #[cfg(feature = "sqlite")]
            BackendType::SQLite { .. } => Box::new(SQLiteBackend::new(paths, args)?),
            #[cfg(not(feature = "sqlite"))]
            BackendType::SQLite { .. } => {
                return Err("The SQLite backend is not enabled in this build".into())
            }
            BackendType::Memory => Box::new(MemoryBackend::new(paths, args)?),
        };

        Ok(Self {
            backend: Mutex::new(backend),
        })
    }
}

declare_plugin!(OpenAddressesPlugin);
