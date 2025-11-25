use std::{env, time::Duration};

pub async fn set_up_database() -> sqlx::PgPool {
    let database_url = env::var("DATABASE_URL").expect("there is no DATABASE_URL environment variable present");
    sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("can't connect to database")

    // sqlx::migrate!("./migrations")
    //     .run(&database_pool)
    //     .await
    //     .expect("cannot run migrations");

}
