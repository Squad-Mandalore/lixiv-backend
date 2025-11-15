use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    prelude::StableGraph,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Logical type of a JSON value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum JsonType {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

impl JsonType {
    pub fn of(value: &Value) -> Self {
        match value {
            Value::Null => JsonType::Null,
            Value::Bool(_) => JsonType::Bool,
            Value::Number(_) => JsonType::Number,
            Value::String(_) => JsonType::String,
            Value::Array(_) => JsonType::Array,
            Value::Object(_) => JsonType::Object,
        }
    }
}

/// Definition of a "kind" e.g. (Team, Application, Technology)
///
/// This defines the schema of the kind, it says which fields an instance of this
/// kind has and which JSON type each field should carry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KindDefinition {
    pub name: String,
    pub parent: Option<String>,
    pub fields: HashMap<String, JsonType>,
}

/// A single node in the graph.
/// It has a unique ID
/// And a kind which determines the structure
/// of the data field
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeInstance {
    pub kind: String,
    pub data: HashMap<String, Value>,
}

impl NodeInstance {
    pub fn get(&self, field: &str) -> Option<&Value> {
        self.data.get(field)
    }
    pub fn new(kind: String, name: String) -> Self {
        let mut init = HashMap::new();
        init.insert("name".into(), Value::String(name));
        NodeInstance { kind, data: init }
    }
    pub fn get_name(&self) -> &String {
        if let Value::String(name) = self.get("name").unwrap() {
            return name;
        }
        unreachable!("name missing")
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
            .find(|node| self[*node].get_name() == weight.get_name());
        match node {
            Some(n) => n,
            None => self.add_node(weight),
        }
    }
}

/// In‑memory registry for all known kinds.
/// This registry will be persisted inside a real database later on
#[derive(Debug, Default)]
pub struct KindRegistry {
    kinds: HashMap<String, KindDefinition>,
}

#[derive(Debug)]
pub enum ValidationError {
    UnknownKind(String),
    UnknownField {
        field: String,
    },
    WrongType {
        field: String,
        expected: JsonType,
        actual: JsonType,
    },
}

impl KindRegistry {
    pub fn register_kind(&mut self, kind: KindDefinition) -> Result<(), String> {
        if let Some(parent_name) = &kind.parent
            && !self.kinds.contains_key(parent_name)
        {
            return Err(format!("Parent kind `{parent_name}` is not registered"));
        }

        if self.kinds.contains_key(&kind.name) {
            return Err(format!("Kind `{}` is already registered", kind.name));
        }

        self.kinds.insert(kind.name.clone(), kind);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&KindDefinition> {
        self.kinds.get(name)
    }

    /// Check that a dynamic instance matches the schema of its kind.
    pub fn validate_instance(&self, instance: &NodeInstance) -> Result<(), ValidationError> {
        let kind = self
            .get(&instance.kind)
            .ok_or_else(|| ValidationError::UnknownKind(instance.kind.clone()))?;

        for (field_name, value) in &instance.data {
            let expected =
                kind.fields
                    .get(field_name)
                    .ok_or_else(|| ValidationError::UnknownField {
                        field: field_name.clone(),
                    })?;

            let actual = JsonType::of(value);
            if actual != *expected {
                return Err(ValidationError::WrongType {
                    field: field_name.clone(),
                    expected: *expected,
                    actual,
                });
            }
        }

        Ok(())
    }
}
