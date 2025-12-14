use crate::model::DbNode;

#[derive(Default)]
pub struct Node;

#[async_graphql::Object]
impl Node {
    async fn nodes(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Vec<DbNode>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let nodes = sqlx::query_as!(
            DbNode,
            r#"
            SELECT id, schema_title, name, data as "data: serde_json::Value", created_at, updated_at
            FROM nodes
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(nodes)
    }

    async fn node(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<Option<DbNode>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let node = sqlx::query_as!(
            DbNode,
            r#"
            SELECT id, schema_title, name, data as "data: serde_json::Value", created_at, updated_at
            FROM nodes
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(node)
    }
}

#[derive(Default)]
pub struct NodeMutation;

#[async_graphql::Object]
impl NodeMutation {
    async fn create_node(
        &self,
        ctx: &async_graphql::Context<'_>,
        schema_title: String,
        name: String,
        data: serde_json::Value,
    ) -> Result<DbNode, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let node = sqlx::query_as!(
            DbNode,
            r#"
            INSERT INTO nodes (schema_title, name, data)
            VALUES ($1, $2, $3)
            RETURNING id, schema_title, name, data as "data: serde_json::Value", created_at, updated_at
            "#,
            schema_title,
            name,
            data
        )
        .fetch_one(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(node)
    }

    async fn delete_node(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<bool, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let result = sqlx::query!(
            r#"
            DELETE FROM nodes
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}
