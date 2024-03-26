use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgConnection;
use actix_web::{web, App, HttpServer};
use crate::routes::{health_check, subscribe};

// We need to mark `run` as public
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(
    listener: TcpListener,
    connection: PgConnection
) -> Result<Server, std::io::Error> {
    // Warp the connection in a smart pointer
    let connection = web::Data::new(connection);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST /subscriptions requests
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the application state
            // Get a pointer copy and attach it to the application state
            .app_data(connection.clone())
    })
        .listen(listener)?
        .run();
    // No .await here!
    Ok(server)
}