use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};

pub async fn connect() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connecting to database: {}", db_url);

    let mut connect_opts = PgConnectOptions::new();
    connect_opts.log_statements(tide::log::LevelFilter::Debug);

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        // .connect_with(connect_opts.database(&db_url))
        .await
        .expect("Failed to connect to database")
}
