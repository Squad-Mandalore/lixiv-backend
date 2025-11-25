use axum::{Extension, Router, middleware, routing::post};
use lixiv_backend::{database::set_up_database, graphql::{create_schema, graphql_handler}};
use sqlx::PgPool;
use tokio::signal;
use tower_http::cors;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(debug_assertions)]
async fn graphql_playground() -> impl axum::response::IntoResponse {
    use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
    axum::response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[tokio::main]
async fn main() {
    // setup tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // setup database connection pool
    let database_pool = set_up_database().await;
    let app = app(database_pool);

    #[cfg(debug_assertions)]
    let app = debug_route(app);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn app(database_pool: PgPool) -> Router {
    let schema = create_schema(database_pool.clone());

    let cors = cors::CorsLayer::new()
        // allow `POST` when accessing the resource
        .allow_methods([hyper::Method::POST])
        .allow_headers([http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
        // allow requests from any origin
        .allow_origin(cors::Any);

    // build our application with a single route
    Router::new()
        .route(
            "/graphql",
            post(graphql_handler)//.layer(middleware::from_fn(auth)),
        )
        .layer(Extension(schema))
        // .route("/login", post(login))
        // .route("/refresh", post(refresh))
        .with_state(database_pool)
        .layer(cors)
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[cfg(debug_assertions)]
fn debug_route(app: Router) -> Router {
    use axum::routing::get;
    let debug = Router::new()
        .route("/playground", get(graphql_playground));

    app.nest("/debug", debug)
}
