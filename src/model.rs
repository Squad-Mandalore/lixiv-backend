// use jsonschema::{ValidationError, paths::Location};
// use petgraph::{
//     graph::{EdgeIndex, NodeIndex},
//     prelude::StableGraph,
// };
// use serde::{Deserialize, Serialize};
use serde_json::{Value}; // Map,
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct DbSchema {
    pub id: i32,
    pub title: String,
    pub schema_json: Value,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow)]
pub struct DbNode {
    pub id: i32,
    pub schema_title: String,
    pub name: String,
    pub data: Value,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow)]
pub struct DbEdge {
    pub id: i32,
    pub source_node_id: i32,
    pub target_node_id: i32,
    pub weight: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_graphql::Object]
impl DbSchema {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn schema_json(&self) -> &Value {
        &self.schema_json
    }

    async fn created_at(&self) -> Option<String> {
        self.created_at.map(|dt| dt.to_rfc3339())
    }

    async fn updated_at(&self) -> Option<String> {
        self.updated_at.map(|dt| dt.to_rfc3339())
    }
}

#[async_graphql::Object]
impl DbNode {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn schema_title(&self) -> &str {
        &self.schema_title
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn data(&self) -> &Value {
        &self.data
    }

    async fn created_at(&self) -> Option<String> {
        self.created_at.map(|dt| dt.to_rfc3339())
    }

    async fn updated_at(&self) -> Option<String> {
        self.updated_at.map(|dt| dt.to_rfc3339())
    }
}

#[async_graphql::Object]
impl DbEdge {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn source_node_id(&self) -> i32 {
        self.source_node_id
    }

    async fn target_node_id(&self) -> i32 {
        self.target_node_id
    }

    async fn weight(&self) -> &str {
        &self.weight
    }

    async fn created_at(&self) -> Option<String> {
        self.created_at.map(|dt| dt.to_rfc3339())
    }
}


//type Schema = Value;

///// Database-backed registry for all known schemas.
//#[derive(Debug)]
//pub struct SchemaRegistry {
//    pool: sqlx::PgPool,
//}

//impl SchemaRegistry {
//    pub fn new(pool: sqlx::PgPool) -> Self {
//        Self { pool }
//    }

//    pub async fn insert<'a>(&self, schema: &'a Schema) -> Result<(), ValidationError<'a>> {
//        jsonschema::meta::validate(schema)?;
//        let title = schema.get("title").and_then(Value::as_str).ok_or_else(|| {
//            ValidationError::custom(
//                Location::new(), // instance location
//                Location::new(), // schema location
//                schema,
//                "schema must contain a string field 'title'",
//            )
//        })?;

//        sqlx::query!(
//            r#"
//            INSERT INTO schemas (title, schema_json)
//            VALUES ($1, $2)
//            ON CONFLICT (title) DO NOTHING
//            "#,
//            title,
//            schema
//        )
//        .execute(&self.pool)
//        .await
//        .map_err(|e| {
//            ValidationError::custom(
//                Location::new(),
//                Location::new(),
//                schema,
//                format!("Database error: {}", e),
//            )
//        })?;

//        Ok(())
//    }

//    pub async fn get(&self, name: &str) -> Result<Option<Schema>, sqlx::Error> {
//        let result = sqlx::query!(
//            r#"
//            SELECT schema_json as "schema_json: Value"
//            FROM schemas
//            WHERE title = $1
//            "#,
//            name
//        )
//        .fetch_optional(&self.pool)
//        .await?;

//        Ok(result.map(|r| r.schema_json))
//    }

//    /// Check that a dynamic instance matches the schema of its kind.
//    pub async fn validate_instance<'a>(
//        &self,
//        instance: &'a NodeInstance,
//    ) -> Result<(), ValidationError<'a>> {
//        let schema = self.get(&instance.schema).await.map_err(|e| {
//            ValidationError::custom(
//                Location::new(), // instance location
//                Location::new(), // schema location
//                &instance.data,
//                format!("Database error: {}", e),
//            )
//        })?;

//        let kind = schema.ok_or_else(|| {
//            ValidationError::custom(
//                Location::new(), // instance location
//                Location::new(), // schema location
//                &instance.data,
//                "schema must contain a string field 'title'",
//            )
//        })?;

//        jsonschema::validate(&kind, &instance.data)?;
//        Ok(())
//    }
//}

///// A single node in the graph.
///// And a schema which determines the structure
///// of the data field
//#[derive(Debug, Clone, Serialize)]
//pub struct NodeInstance {
//    schema: String,
//    data: Value,
//}

//impl NodeInstance {
//    pub fn new(schema: impl Into<String>, name: impl Into<String>) -> Self {
//        let mut init = Map::new();
//        init.insert("name".into(), Value::String(name.into()));

//        NodeInstance {
//            schema: schema.into(),
//            data: Value::Object(init),
//        }
//    }

//    pub fn schema(&self) -> &String {
//        &self.schema
//    }

//    fn data(&self) -> &Map<String, Value> {
//        self.data
//            .as_object()
//            .expect("NodeInstance invariant: data is always an object")
//    }

//    fn data_mut(&mut self) -> &mut Map<String, Value> {
//        self.data
//            .as_object_mut()
//            .expect("NodeInstance invariant: data is always an object")
//    }

//    pub fn get(&self, field: &str) -> Option<&Value> {
//        self.data().get(field)
//    }

//    pub fn get_mut(&mut self, field: &str) -> Option<&mut Value> {
//        self.data_mut().get_mut(field)
//    }

//    pub fn name(&self) -> &str {
//        self.get("name")
//            .and_then(Value::as_str)
//            .expect("name missing or not a string")
//    }

//    pub fn insert(&mut self, k: impl Into<String>, v: Value) -> Option<Value> {
//        self.data_mut().insert(k.into(), v)
//    }
//}

//impl<'de> Deserialize<'de> for NodeInstance {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//    where
//        D: serde::Deserializer<'de>,
//    {
//        #[derive(Deserialize)]
//        struct RawNodeInstance {
//            schema: String,
//            data: Value,
//        }

//        let raw = RawNodeInstance::deserialize(deserializer)?;

//        if !raw.data.is_object() {
//            return Err(serde::de::Error::custom(
//                "NodeInstance.data must be a JSON object",
//            ));
//        }
//        //TODO: look up if raw.schema exists in SchemaRegistry database

//        Ok(NodeInstance {
//            schema: raw.schema,
//            data: raw.data,
//        })
//    }
//}

//pub trait AddDedup {
//    fn add_edge_s(&mut self, source: NodeIndex, target: NodeIndex, weight: String) -> EdgeIndex;
//    fn add_node_s(&mut self, weight: NodeInstance) -> NodeIndex;
//}

//impl AddDedup for StableGraph<NodeInstance, String> {
//    fn add_edge_s(&mut self, source: NodeIndex, target: NodeIndex, weight: String) -> EdgeIndex {
//        let edge = self.find_edge(source, target);
//        match edge {
//            Some(e) => e,
//            None => self.add_edge(source, target, weight),
//        }
//    }

//    fn add_node_s(&mut self, weight: NodeInstance) -> NodeIndex {
//        let node = self
//            .node_indices()
//            .find(|node| self[*node].name() == weight.name());
//        match node {
//            Some(n) => n,
//            None => self.add_node(weight),
//        }
//    }
//}
