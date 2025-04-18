mod admin_dashboard;
mod change_password;
mod health_check;
mod helpers;
mod login;
mod newsletter;
mod subscriptions;
mod subscriptions_confirm;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);

    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to get configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
