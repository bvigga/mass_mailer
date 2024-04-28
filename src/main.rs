use sqlx::postgres::PgPool;
use std::net::TcpListener;
use mass_mailer::configuration::get_configuration;
use mass_mailer::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let address = TcpListener::bind(address)?;
    run(address, connection_pool)?.await?;
    Ok(())
}


