use jsonschema::{ValidationError, paths::Location};
use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    prelude::StableGraph,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

type Schema = Value;

/// In‑memory registry for all known schemas.
/// TODO: This registry will be persisted inside a real database later on
#[derive(Debug, Default)]
pub struct SchemaRegistry {
    schemas: HashMap<String, Schema>,
}

impl SchemaRegistry {
    pub fn insert<'a>(&mut self, schema: &'a Schema) -> Result<(), ValidationError<'a>> {
        jsonschema::meta::validate(schema)?;
        let title = schema.get("title").and_then(Value::as_str).ok_or_else(|| {
            ValidationError::custom(
                Location::new(), // instance location
                Location::new(), // schema location
                schema,
                "schema must contain a string field 'title'",
            )
        })?;
        self.schemas.insert(title.into(), schema.clone());
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Schema> {
        self.schemas.get(name)
    }

    /// Check that a dynamic instance matches the schema of its kind.
    pub fn validate_instance<'a>(
        &self,
        instance: &'a NodeInstance,
    ) -> Result<(), ValidationError<'a>> {
        let kind = self.get(&instance.schema).ok_or_else(|| {
            ValidationError::custom(
                Location::new(), // instance location
                Location::new(), // schema location
                &instance.data,
                "schema must contain a string field 'title'",
            )
        })?;
        jsonschema::validate(kind, &instance.data)?;
        Ok(())
    }
}

/// A single node in the graph.
/// And a schema which determines the structure
/// of the data field
#[derive(Debug, Clone, Serialize)]
pub struct NodeInstance {
    schema: String,
    data: Value,
}

impl NodeInstance {
    pub fn new(schema: impl Into<String>, name: impl Into<String>) -> Self {
        let mut init = Map::new();
        init.insert("name".into(), Value::String(name.into()));

        NodeInstance {
            schema: schema.into(),
            data: Value::Object(init),
        }
    }

    pub fn schema(&self) -> &String {
        &self.schema
    }

    fn data(&self) -> &Map<String, Value> {
        self.data
            .as_object()
            .expect("NodeInstance invariant: data is always an object")
    }

    fn data_mut(&mut self) -> &mut Map<String, Value> {
        self.data
            .as_object_mut()
            .expect("NodeInstance invariant: data is always an object")
    }

    pub fn get(&self, field: &str) -> Option<&Value> {
        self.data().get(field)
    }

    pub fn get_mut(&mut self, field: &str) -> Option<&mut Value> {
        self.data_mut().get_mut(field)
    }

    pub fn name(&self) -> &str {
        self.get("name")
            .and_then(Value::as_str)
            .expect("name missing or not a string")
    }

    pub fn insert(&mut self, k: impl Into<String>, v: Value) -> Option<Value> {
        self.data_mut().insert(k.into(), v)
    }
}

impl<'de> Deserialize<'de> for NodeInstance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawNodeInstance {
            schema: String,
            data: Value,
        }

        let raw = RawNodeInstance::deserialize(deserializer)?;

        if !raw.data.is_object() {
            return Err(serde::de::Error::custom(
                "NodeInstance.data must be a JSON object",
            ));
        }
        //TODO: look up if raw.schema exists in SchemaRegistry database

        Ok(NodeInstance {
            schema: raw.schema,
            data: raw.data,
        })
    }
}
pub trait AddDedup {
    fn add_edge_s(&mut self, source: NodeIndex, target: NodeIndex, weight: String) -> EdgeIndex;
    fn add_node_s(&mut self, weight: NodeInstance) -> NodeIndex;
}

impl AddDedup for StableGraph<NodeInstance, String> {
    fn add_edge_s(&mut self, source: NodeIndex, target: NodeIndex, weight: String) -> EdgeIndex {
        let edge = self.find_edge(source, target);
        match edge {
            Some(e) => e,
            None => self.add_edge(source, target, weight),
        }
    }

    fn add_node_s(&mut self, weight: NodeInstance) -> NodeIndex {
        let node = self
            .node_indices()
            .find(|node| self[*node].name() == weight.name());
        match node {
            Some(n) => n,
            None => self.add_node(weight),
        }
    }
}
