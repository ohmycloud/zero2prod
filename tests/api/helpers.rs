use once_cell::sync::Lazy;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::{get_connection_pool, Application};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

// Ensure that the `tracing` stack is only initialized once using `once_celll`
static TRACING: Lazy<()> = Lazy::new(|| {});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let configuration = get_configuration().expect("Failed to get configuration");
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    // Got the port before spawning the application
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
    }
}
