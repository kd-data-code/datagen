#[cfg(feature = "generate")]
use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::generate::generated_schema::IntoRandom;
#[cfg(feature = "generate")]
use crate::plugins::plugin_list::PluginList;
#[cfg(any(feature = "schema", any(feature = "serialize", feature = "generate")))]
use crate::schema::schema_definition::Schema;
#[cfg(feature = "generate")]
use crate::schema::schema_definition::Serializer;
#[cfg(any(feature = "schema", feature = "serialize"))]
use crate::util::types::Result;
#[cfg(feature = "schema")]
use schemars::schema_for;
#[cfg(feature = "generate")]
use serde_json::Value;
#[cfg(any(feature = "schema", feature = "serialize"))]
use std::fs::File;
#[cfg(any(feature = "schema", feature = "serialize"))]
use std::path::Path;
#[cfg(feature = "generate")]
use std::sync::Arc;

#[cfg(feature = "schema")]
pub fn write_json_schema<P: AsRef<Path>>(path: P) -> Result<()> {
    let file = File::create(path)?;
    let schema = schema_for!(Schema);

    serde_json::to_writer_pretty(file, &schema).map_err(|e| e.into())
}

#[cfg(feature = "serialize")]
pub fn read_schema<P: AsRef<Path>>(path: P) -> Result<Schema> {
    let file = File::open(path)?;
    let schema: Schema = serde_json::from_reader(file)?;

    Ok(schema)
}

#[cfg(feature = "generate")]
pub fn generate_random_data(schema: Schema) -> Result<String> {
    let plugins = PluginList::from_schema(&schema)?;
    let options = Arc::new(schema.options.unwrap_or_default());
    let root = CurrentSchema::root(options.clone(), plugins.clone());
    let generated = schema.value.into_random(root)?;

    match options.serializer.as_ref().unwrap_or_default() {
        Serializer::Json { pretty } => pretty
            .unwrap_or(false)
            .then(|| serde_json::to_string_pretty(&generated))
            .unwrap_or_else(|| serde_json::to_string(&generated))
            .map_err(Into::into),
        Serializer::Yaml { pretty } => pretty
            .unwrap_or(false)
            .then(|| serde_yaml::to_string(&generated))
            .unwrap_or_else(|| serde_yaml::to_string(&generated))
            .map_err(Into::into),
        Serializer::Xml { root_element } => {
            quick_xml::se::to_string_with_root(root_element, &generated).map_err(Into::into)
        }
        Serializer::Plugin { plugin_name, args } => plugins
            .get(plugin_name)?
            .serialize(&generated, args.clone().unwrap_or(Value::Null)),
    }
}