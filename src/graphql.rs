use async_graphql::{EmptySubscription, MergedObject, Schema, extensions::Logger};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;

#[derive(MergedObject, Default)]
pub struct Query(
);

#[derive(MergedObject, Default)]
pub struct Mutation(
);

pub type SchemaType = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(database_pool: sqlx::PgPool) -> SchemaType {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(Logger)
        .data(database_pool)
        .finish()
}

pub async fn graphql_handler(
    schema: Extension<SchemaType>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(
            request
                .into_inner()
        )
        .await
        .into()
}
