use crate::generate::current_schema::CurrentSchema;
#[cfg(feature = "generate")]
use crate::schema::transform::Transform;
use crate::util::types::Result;
use indexmap::IndexMap;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", serde(untagged))]
pub enum GeneratedSchema {
    None,
    Number(f64),
    Integer(i32),
    Bool(bool),
    String(String),
    Array(Vec<Arc<GeneratedSchema>>),
    Object(IndexMap<String, Arc<GeneratedSchema>>),
    Value(Value),
}

#[cfg(feature = "generate")]
pub(crate) trait IntoGenerated: Sized {
    fn into_generated(self, schema: Arc<CurrentSchema>) -> Result<GeneratedSchema>;

    fn get_transform(&self) -> Option<Transform>;

    fn should_finalize(&self) -> bool {
        true
    }
}

#[cfg(feature = "generate")]
pub(crate) trait IntoGeneratedArc: Sized {
    fn into_generated_arc(self, schema: Arc<CurrentSchema>) -> Result<Arc<GeneratedSchema>>;

    fn get_transform(&self) -> Option<Transform>;

    fn should_finalize(&self) -> bool {
        true
    }
}

#[cfg(feature = "generate")]
impl<T> IntoGeneratedArc for T
where
    T: IntoGenerated,
{
    fn into_generated_arc(self, schema: Arc<CurrentSchema>) -> Result<Arc<GeneratedSchema>> {
        Ok(Arc::new(self.into_generated(schema)?))
    }

    fn get_transform(&self) -> Option<Transform> {
        self.get_transform()
    }

    fn should_finalize(&self) -> bool {
        self.should_finalize()
    }
}

pub trait IntoRandom {
    fn into_random(self, schema: Arc<CurrentSchema>) -> Result<Arc<GeneratedSchema>>;
}

#[cfg(feature = "generate")]
impl<T> IntoRandom for T
where
    T: IntoGeneratedArc,
{
    fn into_random(self, schema: Arc<CurrentSchema>) -> Result<Arc<GeneratedSchema>> {
        let transform = self.get_transform();
        let should_finalize = self.should_finalize();

        let mut res = self.into_generated_arc(schema.clone())?;
        if let Some(transform) = transform {
            res = transform.transform(schema.clone(), res)?;
        }

        Ok(if should_finalize {
            schema.finalize(res)
        } else {
            res
        })
    }
}