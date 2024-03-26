//! main.rs

use std::net::TcpListener;
use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgConnection::connect(
        &configuration.database.connection_string()
    ).await
     .expect("Failed to connect to Postgres.");
    run(listener, connection)?.await
}
