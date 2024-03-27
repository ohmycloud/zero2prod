//! main.rs

use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use env_logger::Env;
use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

pub fn get_subscriber(
    name: String,
    env_filter: String
) -> impl Subscriber + Send + Sync {
    /// Compose multiple layers into a `tracing`'s subscriber.
    ///
    /// Implementation Notes
    ///
    /// We are using `impl Subscriber` as return type to avoid having to
    /// spell out the actual type of the returned subscriber, which is
    /// indeed quite complex
    /// We need to explicitly call out that the returned subscriber is
    /// `Send` and `Sync` to make it possible to pass it to `init_subscriber` later on
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        // Output the formatted spans to stdout
        std::io::stdout
    );

    // The `with` method is provided by `SubscriberExt`, an extension trait for
    // `Subscriber` exposed by `tracing_subscriber`
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool= PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    run(listener, connection_pool)?.await
}
