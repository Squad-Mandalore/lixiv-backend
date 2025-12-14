use crate::model::DbSchema;

#[derive(Default)]
pub struct Schema;

#[async_graphql::Object]
impl Schema {
    async fn schemas(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Result<Vec<DbSchema>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let schemas = sqlx::query_as!(
            DbSchema,
            r#"
            SELECT id, title, schema_json as "schema_json: serde_json::Value", created_at, updated_at
            FROM schemas
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(schemas)
    }

    async fn schema(
        &self,
        ctx: &async_graphql::Context<'_>,
        title: String,
    ) -> Result<Option<DbSchema>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let schema = sqlx::query_as!(
            DbSchema,
            r#"
            SELECT id, title, schema_json as "schema_json: serde_json::Value", created_at, updated_at
            FROM schemas
            WHERE title = $1
            "#,
            title
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(schema)
    }
}

#[derive(Default)]
pub struct SchemaMutation;

#[async_graphql::Object]
impl SchemaMutation {
    async fn create_schema(
        &self,
        ctx: &async_graphql::Context<'_>,
        title: String,
        schema_json: serde_json::Value,
    ) -> Result<DbSchema, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let schema = sqlx::query_as!(
            DbSchema,
            r#"
            INSERT INTO schemas (title, schema_json)
            VALUES ($1, $2)
            RETURNING id, title, schema_json as "schema_json: serde_json::Value", created_at, updated_at
            "#,
            title,
            schema_json
        )
        .fetch_one(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(schema)
    }

    async fn delete_schema(
        &self,
        ctx: &async_graphql::Context<'_>,
        title: String,
    ) -> Result<bool, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let result = sqlx::query!(
            r#"
            DELETE FROM schemas
            WHERE title = $1
            "#,
            title
        )
        .execute(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}
