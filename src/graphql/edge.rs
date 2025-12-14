use crate::model::DbEdge;

#[derive(Default)]
pub struct Edge;

#[async_graphql::Object]
impl Edge {
    async fn edges(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Vec<DbEdge>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let edges = sqlx::query_as!(
            DbEdge,
            r#"
            SELECT id, source_node_id, target_node_id, weight, created_at
            FROM edges
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(edges)
    }
}

#[derive(Default)]
pub struct EdgeMutation;

#[async_graphql::Object]
impl EdgeMutation {
    async fn create_edge(
        &self,
        ctx: &async_graphql::Context<'_>,
        source_node_id: i32,
        target_node_id: i32,
        weight: String,
    ) -> Result<DbEdge, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let edge = sqlx::query_as!(
            DbEdge,
            r#"
            INSERT INTO edges (source_node_id, target_node_id, weight)
            VALUES ($1, $2, $3)
            RETURNING id, source_node_id, target_node_id, weight, created_at
            "#,
            source_node_id,
            target_node_id,
            weight
        )
        .fetch_one(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(edge)
    }

    async fn delete_edge(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<bool, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let result = sqlx::query!(
            r#"
            DELETE FROM edges
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
